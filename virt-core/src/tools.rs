use crate::geometry::{Shape, Vector};
use crate::action::Action;

#[derive(Debug)]
pub struct Button {
    pub shape: Box<dyn Shape>,

    pub action: Option<Action>,
}

impl Button {
    pub fn new(shape: Box<dyn Shape>, action: Option<Action>) -> Button {
        Button {
            shape,
            action,
        }
    }

    pub fn clicked(&mut self, v: Vector) {
        match &mut self.action {
            Some(a) => {
                if self.shape.contains(v) {
                    a.run().unwrap();
                }
            },
            None => {}
        }
    }
}