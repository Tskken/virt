use vulkano::command_buffer::DynamicState;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice, PhysicalDeviceType};
use vulkano::pipeline::viewport::Viewport;
use winit::window::Window;

use std::sync::Arc;
use std::{env, path::PathBuf};
use glob::{glob, Paths};
use toml;
use serde_derive::Deserialize;
use std::fs;

use crate::error::{CoreError, Result};

pub fn find_device_index(instance: Arc<Instance>, ty: PhysicalDeviceType) -> Result<usize> {
    for device in PhysicalDevice::enumerate(&instance) {
        if device.ty() == ty {
            return Ok(device.index())
        }
    };

    Err(CoreError::NoSupportedPhysicalDevice)
}

pub fn widget_paths(cfg: CoreConfig) -> Result<Paths> {
    match cfg.root_path {
        Some(path) => {
            let full_path = path.join("**/*.toml");
            Ok(glob(full_path.to_str().unwrap())?)
        },
        None => {
            let env_path = env::current_dir()?;
            let full_path = env_path.join("widgets/**/*.toml");
            Ok(glob(full_path.to_str().unwrap())?)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoreConfig {
    root_path: Option<PathBuf>,
}

impl CoreConfig {
    pub fn new() -> Result<CoreConfig> {
        let env_path = env::current_dir()?;
        let full_path = env_path.join("config.toml");
        let data = fs::read_to_string(full_path)?;
        let cfg: CoreConfig = toml::from_str(&data)?;

        Ok(cfg)
    }
}

pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub enum Color {}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
        [r, g, b, a]
    }

    pub fn none() -> [f32; 4] {
        [0f32, 0f32, 0f32, 0f32]
    }

    pub fn from_hex(data: Vec<u8>) -> [f32; 4] {
        let a = data[3] as f32 / u8::MAX as f32;
        [
            (data[0] as f32 / u8::MAX as f32) * a,
            (data[1] as f32 / u8::MAX as f32) * a,
            (data[2] as f32 / u8::MAX as f32) * a,
            a,
        ]
    }
}
