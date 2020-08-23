use toml;
use serde_derive::Deserialize;
use std::fs;
use std::{env, path::PathBuf};
use glob::{glob, Paths};

use crate::error::Result;

pub fn decode(path: &str) -> Result<WidgetConfig> {

    let data = fs::read_to_string(path)?;

    let widget_config: WidgetConfig = toml::from_str(&data)?;

    Ok(widget_config)
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

#[derive(Debug, Clone, Deserialize)]
pub struct WidgetConfig {
    // name of WidgetConfig <Optional>
    pub name: Option<String>,
    
    // Dimensions of window, width and height <Not-Optional>
    pub width: u32,
    pub height: u32,

    // Position of top left of window on the screen
    pub position: [f32; 2],

    // Hex color value for window background <Optional>
    pub color: Option<String>,

    // Lits of shapes to be drawn to the WidgetConfig <Optional>
    pub shape: Option<Vec<ShapeWidgetConfig>>,

    pub tool: Option<Vec<ToolWidgetConfig>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShapeWidgetConfig {
    // Type of shape to be drawn <Non-Optional>
    pub shape_type: Shapes,

    pub shape: Vec<f32>,

    // Hex color value for shape background <Optional>
    pub color: Option<String>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Shapes {
    Triangle,
    Rectangle,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolWidgetConfig {
    pub tool_type: Tools,

    pub shape: Vec<f32>,

    pub color: Option<String>,

    pub action: Option<String>,

    pub args: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Tools {
    Button,
}
