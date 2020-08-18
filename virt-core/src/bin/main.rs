//#![windows_subsystem = "windows"]

#![allow(deprecated)]

use virt_core::core::CoreState;
use winit::event::{Event, WindowEvent, ElementState, MouseButton};
use winit::event_loop::ControlFlow;
//use winit::dpi::PhysicalPosition;

use virt_core::geometry::Vector;

fn main() {
    let (mut core_state, event_loop) = CoreState::new();

    //let mut mouse_pos: Option<PhysicalPosition<f64>> = None;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(_),
                ..
            } => {
                core_state.surfaces
                    .get_mut(&window_id)
                    .unwrap()
                    .recreate_swapchain = true;
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    modifiers: _,
                },
                ..
            } => {
                let surface = core_state.surfaces.get_mut(&window_id).unwrap();

                match surface.cur_mouse_pos {
                    Some(val) => {
                        surface.las_mouse_pos = Some(val);
                        surface.cur_mouse_pos = Some(Vector::new(position.x as f32, position.y as f32).project(surface.widget.width as f32, surface.widget.height as f32));
                    },
                    None => {
                        surface.cur_mouse_pos = Some(Vector::new(position.x as f32, position.y as f32).project(surface.widget.width as f32, surface.widget.height as f32));
                    }
                }

                
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    modifiers: _,
                },
                ..
            } => {
                if state == ElementState::Released && button == MouseButton::Left {
                    let surface = core_state.surfaces.get_mut(&window_id).unwrap();
                    match surface.cur_mouse_pos {
                        Some(val) => {
                            for button in &mut surface.widget.buttons {
                                button.clicked(val);
                            };
                        },
                        None => {},
                    }
                };
            }
            Event::RedrawEventsCleared => {
                core_state.surfaces
                    .values()
                    .for_each(|s| s.surface.window().request_redraw());
            }
            Event::RedrawRequested(window_id) => {
                core_state.draw(window_id);
            }
            _ => (),
        }
    });
}