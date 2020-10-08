use toml;
use serde_derive::Deserialize;
use std::fs;

use crate::error::Result;

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
    pub width: f32,
    pub height: f32,

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

    pub format: Option<Format>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Shapes {
    Triangle,
    Rectangle,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Format {
    Fill,
    Line,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolWidgetConfig {
    pub ty: Tools,

    pub shape: Vec<f32>,

    pub color: Option<String>,

    pub action: Option<Action>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Tools {
    Button,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Action {
    pub action: String,

    pub args: Option<Vec<String>>,

    pub ty: Type,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Type {
    Clicked,
}
