use crate::integration::winit_app::{make_window, run_app, WinitAppBuilder};
use crate::prelude::*;
use log::error;
use std::num::NonZeroU32;
use std::ops::Deref;
use winit::event::{ElementState, Event, KeyEvent, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::PhysicalKey;

/// Create and run a loop using Softbuffer and Winit
///
/// If you want to use [Scene][scenes::Scene]s consider [run_scenes][scenes::run_scenes]
///
/// # Arguments
/// * `width` - Width of the whole window canvas in pixels
/// * `height` - Height of the whole window canvas in pixels
/// * `title` - Window title
/// * `system` - Your program
/// * `options` - [Options] controls how fast the program can update, [UiElement] styling, etc
///
/// # Returns
///
/// Returns when the program is finished executing either due to it quitting or a fatal error occurring
pub fn run(
    width: usize,
    height: usize,
    title: &str,
    system: Box<dyn System>,
    options: Options,
) -> Result<(), GraphicsError> {
    let event_loop = EventLoop::new().unwrap();
    let title = title.to_string();
    let app = WinitAppBuilder::new(system, options, move |elwt, system, options| {
        elwt.set_control_flow(options.control_flow);
        let (scale, window) = make_window(elwt, system, &options, width, height, title.clone())
            .expect("Window created");

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        let size = window.inner_size();
        if let (Some(win_width), Some(win_height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        {
            surface
                .resize(win_width, win_height)
                .expect("Resized softbuffer");
        }
        (scale, window, surface)
    })
    .setup(move |state, event, elwt, system, timing, mouse, options| {
        let (scale, window, surface) = state;

        timing.update();
        timing.accumulated_time += timing.delta;
        while timing.accumulated_time >= timing.fixed_time_step {
            system.update(&timing, window.deref());
            timing.accumulated_time -= timing.fixed_time_step;
            timing.updates += 1;
        }

        if options.control_flow == ControlFlow::Poll {
            if event == Event::AboutToWait {
                window.request_redraw();
            }
        }

        if let Event::WindowEvent { window_id, event } = event {
            if window_id == window.id() {
                match event {
                    WindowEvent::Resized(size) => {
                        if let (Some(win_width), Some(win_height)) =
                            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                        {
                            surface
                                .resize(win_width, win_height)
                                .expect("Resized softbuffer");

                            let horz_scale = win_width.get() as usize / width;
                            let vert_scale = win_height.get() as usize / height;
                            let new_scale: usize = horz_scale.min(vert_scale);
                            *scale = new_scale as f64;
                        }
                    }
                    WindowEvent::CloseRequested => {
                        system.on_window_closed();
                        #[cfg(feature = "window_prefs")]
                        if let Some(mut prefs) = system.window_prefs() {
                            prefs.store(window.deref());
                            //can't return from here so just print out error
                            let _ = prefs
                                .save()
                                .map_err(|err| error!("Unable to save window size/pos: {err:?}"));
                        }
                        elwt.exit();
                    }
                    WindowEvent::Occluded(hidden) => system.on_visibility_changed(!hidden),
                    WindowEvent::Focused(focused) => system.on_focus_changed(focused),
                    WindowEvent::KeyboardInput {
                        device_id: _device_id,
                        event,
                        is_synthetic: _is_synthetic,
                    } => match event {
                        KeyEvent {
                            physical_key,
                            state,
                            repeat,
                            ..
                        } => {
                            if let PhysicalKey::Code(keycode) = physical_key {
                                match state {
                                    ElementState::Pressed => {
                                        if !repeat {
                                            system.on_key_down(vec![keycode])
                                        }
                                    }
                                    ElementState::Released => system.on_key_up(vec![keycode]),
                                }
                            }
                        }
                    },
                    WindowEvent::RedrawRequested => {
                        let mut buffer = surface.buffer_mut().expect("Accessing softbuffer buffer");
                        let mut drawing_buffer = Graphics::create_buffer_u32(width, height);
                        let mut drawing_graphics =
                            Graphics::new_u32_argb(&mut drawing_buffer, width, height)
                                .expect("Graphics creation");
                        system.render(&mut drawing_graphics);
                        let mut image = drawing_graphics.copy_to_image();
                        if *scale > 1.0 {
                            let factor = scale.trunc() as usize;
                            image = image.scale(
                                Scaling::nearest_neighbour(factor, factor)
                                    .expect("Invalid scaling"),
                            );
                        }
                        let physical_size = window.inner_size();
                        let mut graphics = Graphics::new_u32_argb(
                            &mut buffer,
                            physical_size.width as usize,
                            physical_size.height as usize,
                        )
                        .expect("Graphics creation");
                        graphics.draw_image((0, 0), &image);
                        timing.renders += 1;
                        buffer.present().expect("Softbuffer presented to screen");
                    }
                    WindowEvent::MouseWheel {
                        device_id: _device_id,
                        delta,
                        phase,
                    } => {
                        if phase == TouchPhase::Moved {
                            match delta {
                                MouseScrollDelta::LineDelta(_, _) => {}
                                MouseScrollDelta::PixelDelta(pos) => {
                                    system.on_scroll(
                                        &mouse,
                                        pos.x.round() as isize,
                                        pos.y.round() as isize,
                                    );
                                }
                            }
                        }
                    }
                    WindowEvent::MouseInput {
                        device_id: _device_id,
                        state,
                        button,
                    } => match state {
                        ElementState::Pressed => {
                            mouse.add_down(mouse.xy, button);
                            system.on_mouse_down(&mouse, button);
                        }
                        ElementState::Released => {
                            mouse.add_up(button);
                            system.on_mouse_up(&mouse, button);
                        }
                    },
                    WindowEvent::CursorMoved {
                        device_id: _device_id,
                        position,
                    } => {
                        mouse.xy = coord!(position.x, position.y) / *scale;
                        system.on_mouse_move(&mouse);
                    }
                    _ => {}
                }
            }
        }

        if system.should_exit() {
            elwt.exit();
        }

        timing.update_fps();

        timing.last = timing.now;
    });

    run_app(event_loop, app).map_err(GraphicsError::WinitInit)?;
    Ok(())
}
