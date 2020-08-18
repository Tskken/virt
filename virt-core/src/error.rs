use toml::de;
use std::fmt;
use std::error;
use std::io;
use hex::FromHexError;
use vulkano::instance::InstanceCreationError;
use vulkano::framebuffer::{FramebufferCreationError, RenderPassCreationError};
use vulkano::device::DeviceCreationError;
use vulkano::swapchain::{SwapchainCreationError, AcquireError, CapabilitiesError};
use vulkano::OomError;
use vulkano::command_buffer::{BuildError, CommandBufferExecError, BeginRenderPassError, DrawError};
use vulkano::sync::FlushError;
use vulkano_win::CreationError;
use vulkano::pipeline::GraphicsPipelineCreationError;
use vulkano::memory::DeviceMemoryAllocError;


pub type Result<T> = std::result::Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {
    ValidationFail,
    InvalidShapeFormat,
    NoSupportedPhysicalDevice,
    TomlError(de::Error),
    IoError(io::Error),
    FromHexError(FromHexError),
    InstanceCreationError(InstanceCreationError),
    FramebufferCreationError(FramebufferCreationError),
    DeviceCreationError(DeviceCreationError),
    SwapchainCreationError(SwapchainCreationError),
    AcquireError(AcquireError),
    OomError(OomError),
    BuildError(BuildError),
    CommandBufferExecError(CommandBufferExecError),
    FlushError(FlushError),
    CreationError(CreationError),
    CapabilitiesError(CapabilitiesError),
    RenderPassCreationError(RenderPassCreationError),
    GraphicsPipelineCreationError(GraphicsPipelineCreationError),
    BeginRenderPassError(BeginRenderPassError),
    DrawError(DrawError),
    DeviceMemoryAllocError(DeviceMemoryAllocError)
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CoreError::ValidationFail =>
                write!(f, "validation failed during decoding of WidgetConfig"),
            CoreError::InvalidShapeFormat =>
                write!(f, "the provided values to shape are incorrect"),
            CoreError::NoSupportedPhysicalDevice =>
                write!(f, "no supported physical device found"),
            CoreError::TomlError(ref e) => e.fmt(f),
            CoreError::IoError(ref e) => e.fmt(f),
            CoreError::FromHexError(ref e) => e.fmt(f),
            CoreError::InstanceCreationError(ref e) => e.fmt(f),
            CoreError::FramebufferCreationError(ref e) => e.fmt(f),
            CoreError::DeviceCreationError(ref e) => e.fmt(f),
            CoreError::SwapchainCreationError(ref e) => e.fmt(f),
            CoreError::AcquireError(ref e) => e.fmt(f),
            CoreError::OomError(ref e) => e.fmt(f),
            CoreError::BuildError(ref e) => e.fmt(f),
            CoreError::CommandBufferExecError(ref e) => e.fmt(f),
            CoreError::FlushError(ref e) => e.fmt(f),
            CoreError::CreationError(ref e) => e.fmt(f),
            CoreError::CapabilitiesError(ref e) => e.fmt(f),
            CoreError::RenderPassCreationError(ref e) => e.fmt(f),
            CoreError::GraphicsPipelineCreationError(ref e) => e.fmt(f),
            CoreError::BeginRenderPassError(ref e) => e.fmt(f),
            CoreError::DrawError(ref e) => e.fmt(f),
            CoreError::DeviceMemoryAllocError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for CoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CoreError::ValidationFail => None,
            CoreError::InvalidShapeFormat => None,
            CoreError::NoSupportedPhysicalDevice => None,
            CoreError::TomlError(ref e) => Some(e),
            CoreError::IoError(ref e) => Some(e),
            CoreError::FromHexError(ref e) => Some(e),
            CoreError::InstanceCreationError(ref e) => Some(e),
            CoreError::FramebufferCreationError(ref e) => Some(e),
            CoreError::DeviceCreationError(ref e) => Some(e),
            CoreError::SwapchainCreationError(ref e) => Some(e),
            CoreError::AcquireError(ref e) => Some(e),
            CoreError::OomError(ref e) => Some(e),
            CoreError::BuildError(ref e) => Some(e),
            CoreError::CommandBufferExecError(ref e) => Some(e),
            CoreError::FlushError(ref e) => Some(e),
            CoreError::CreationError(ref e) => Some(e),
            CoreError::CapabilitiesError(ref e) => Some(e),
            CoreError::RenderPassCreationError(ref e) => Some(e),
            CoreError::GraphicsPipelineCreationError(ref e) => Some(e),
            CoreError::BeginRenderPassError(ref e) => Some(e),
            CoreError::DrawError(ref e) => Some(e),
            CoreError::DeviceMemoryAllocError(ref e) => Some(e),
        }
    }
}

impl From<de::Error> for CoreError {
    fn from(err: de::Error) -> CoreError {
        CoreError::TomlError(err)
    }
}

impl From<io::Error> for CoreError {
    fn from(err: io::Error) -> CoreError {
        CoreError::IoError(err)
    }
}

impl From<hex::FromHexError> for CoreError {
    fn from(err: hex::FromHexError) -> CoreError {
        CoreError::FromHexError(err)
    }
}

impl From<InstanceCreationError> for CoreError {
    fn from(err: InstanceCreationError) -> CoreError {
        CoreError::InstanceCreationError(err)
    }
}

impl From<FramebufferCreationError> for CoreError {
    fn from(err: FramebufferCreationError) -> CoreError {
        CoreError::FramebufferCreationError(err)
    }
}

impl From<DeviceCreationError> for CoreError {
    fn from(err: DeviceCreationError) -> CoreError {
        CoreError::DeviceCreationError(err)
    }
}

impl From<SwapchainCreationError> for CoreError {
    fn from(err: SwapchainCreationError) -> CoreError {
        CoreError::SwapchainCreationError(err)
    }
}

impl From<AcquireError> for CoreError {
    fn from(err: AcquireError) -> CoreError {
        CoreError::AcquireError(err)
    }
}

impl From<OomError> for CoreError {
    fn from(err: OomError) -> CoreError {
        CoreError::OomError(err)
    }
}

impl From<BuildError> for CoreError {
    fn from(err: BuildError) -> CoreError {
        CoreError::BuildError(err)
    }
}

impl From<CommandBufferExecError> for CoreError {
    fn from(err: CommandBufferExecError) -> CoreError {
        CoreError::CommandBufferExecError(err)
    }
}

impl From<FlushError> for CoreError {
    fn from(err: FlushError) -> CoreError {
        CoreError::FlushError(err)
    }
}

impl From<CreationError> for CoreError {
    fn from(err: CreationError) -> CoreError {
        CoreError::CreationError(err)
    }
}

impl From<CapabilitiesError> for CoreError {
    fn from(err: CapabilitiesError) -> CoreError {
        CoreError::CapabilitiesError(err)
    }
}

impl From<RenderPassCreationError> for CoreError {
    fn from(err: RenderPassCreationError) -> CoreError {
        CoreError::RenderPassCreationError(err)
    }
}

impl From<GraphicsPipelineCreationError> for CoreError {
    fn from(err: GraphicsPipelineCreationError) -> CoreError {
        CoreError::GraphicsPipelineCreationError(err)
    }
}

impl From<BeginRenderPassError> for CoreError {
    fn from(err: BeginRenderPassError) -> CoreError {
        CoreError::BeginRenderPassError(err)
    }
}

impl From<DrawError> for CoreError {
    fn from(err: DrawError) -> CoreError {
        CoreError::DrawError(err)
    }
}

impl From<DeviceMemoryAllocError> for CoreError {
    fn from(err: DeviceMemoryAllocError) -> CoreError {
        CoreError::DeviceMemoryAllocError(err)
    }
}

