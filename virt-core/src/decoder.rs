use toml;
use serde_derive::Deserialize;
use std::fs;
use toml::de;
use std::fmt;
use std::error;
use std::io;
use hex;

type Result<T> = std::result::Result<T, DecoderError>;

#[derive(Debug)]
pub enum DecoderError {
    ValidationFail,
    InvalidShapeFormat,
    ParseTomlErr(de::Error),
    ParseIoErr(io::Error),
    ParseHexErr(hex::FromHexError),
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecoderError::ValidationFail =>
                write!(f, "validation failed during decoding of WidgetConfig"),
            DecoderError::InvalidShapeFormat =>
                write!(f, "the provided values to shape are incorrect"),
            DecoderError::ParseTomlErr(ref e) => e.fmt(f),
            DecoderError::ParseIoErr(ref e) => e.fmt(f),
            DecoderError::ParseHexErr(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DecoderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DecoderError::ValidationFail => None,
            DecoderError::InvalidShapeFormat => None,
            DecoderError::ParseTomlErr(ref e) => Some(e),
            DecoderError::ParseIoErr(ref e) => Some(e),
            DecoderError::ParseHexErr(ref e) => Some(e),
        }
    }
}

impl From<de::Error> for DecoderError {
    fn from(err: de::Error) -> DecoderError {
        DecoderError::ParseTomlErr(err)
    }
}

impl From<io::Error> for DecoderError {
    fn from(err: io::Error) -> DecoderError {
        DecoderError::ParseIoErr(err)
    }
}

impl From<hex::FromHexError> for DecoderError {
    fn from(err: hex::FromHexError) -> DecoderError {
        DecoderError::ParseHexErr(err)
    }
}


pub fn decode(path: &str) -> Result<WidgetConfig> {

    let data = fs::read_to_string(path)?;

    let widget_config: WidgetConfig = toml::from_str(&data)?;

    Ok(widget_config)
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
