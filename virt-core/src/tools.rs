use crate::geometry::{Shape, Vector};
use crate::action::{Action, ActionType};
use crate::error::Result;

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

    pub fn clicked(&mut self, v: Vector) -> Result<()> {
        match &mut self.action {
            Some(a) => {
                if a.ty == ActionType::Clicked {
                    if self.shape.contains(v) {
                        a.run()?;
                    };
                };
            },
            None => {}
        };

        Ok(())
    }
}