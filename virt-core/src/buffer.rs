use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

use std::sync::Arc;

use crate::vector::Vector;
use crate::error::{CoreError, Result};

#[derive(Debug, Clone)]
pub struct Buffer {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vector]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u16]>>,
}

impl Buffer {
    pub fn new(vertex_buffer: Arc<CpuAccessibleBuffer<[Vector]>>, index_buffer: Arc<CpuAccessibleBuffer<[u16]>>) -> Buffer {
        Buffer {
            vertex_buffer,
            index_buffer,
        }
    }
}