use crate::prelude::*;
use crate::GraphicsError::LoadingWindowPref;
use crate::{GraphicsError, MouseData, Options, System};
use buffer_graphics_lib::Graphics;
use pixels::PixelsBuilder;
use pixels::{Pixels, SurfaceTexture};
use simple_game_utils::prelude::Timing;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::event::{Event, MouseButton, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::CursorGrabMode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn create_window(
    size: (u32, u32),
    title: &str,
    scale: WindowScaling,
    event_loop: &EventLoop<()>,
) -> Result<Window, GraphicsError> {
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .map_err(|err| GraphicsError::WindowInit(format!("{err:?}")))?;
    let factor = match scale {
        WindowScaling::Native => window.scale_factor(),
        WindowScaling::Double => window.scale_factor() + 2.0,
        WindowScaling::Quad => window.scale_factor() + 4.0,
    };

    let px_size: PhysicalSize<u32> = LogicalSize::new(size.0, size.1).to_physical(factor);

    window.set_min_inner_size(Some(px_size));
    let _ = window.request_inner_size(px_size);
    window.set_visible(true);

    Ok(window)
}

/// Creates the window and pixels wrapper
///
/// The inner size mentioned in the arguments refers to the size of the area available to draw in, it doesn't include the window frame, etc
///
/// This uses logical pixels, where on a low DPI screen each library pixel is one display pixel but on higher DPI screens (and if
/// `scale` != `None`) then a library pixel will be represented by multiple display pixels
///
/// # Arguments
///
/// * `canvas_size` - Inner width and height of window in logical pixels
/// * `options` - Scaling, UPS, etc options
/// * `title` - Title for window
/// * `event_loop` - Provided by `EventLoop::new()`, this allows the window to receive events from the OS
///
/// # Example
///
/// This creates a 160x160 window:
///
/// `let (mut window, graphics) = setup(160, 160, "Example", true, &event_loop)?;`
///
/// # Returns
///
/// A result with a pair of Window and PixelsWrapper
///
/// # Errors
///
/// * `WindowInit` - If the window can not be created
fn setup(
    canvas_size: (u32, u32),
    options: &Options,
    title: &str,
    event_loop: &EventLoop<()>,
) -> Result<(Window, Pixels), GraphicsError> {
    let win = create_window(canvas_size, title, options.scaling, event_loop)?;
    let surface = SurfaceTexture::new(win.inner_size().width, win.inner_size().height, &win);
    let pixels = PixelsBuilder::new(canvas_size.0 as u32, canvas_size.1 as u32, surface)
        .enable_vsync(options.vsync)
        .build()
        .map_err(GraphicsError::PixelsInit)?;
    Ok((win, pixels))
}

/// Create and run a loop using Pixels and Winit
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
    mut system: Box<dyn System>,
    options: Options,
) -> Result<(), GraphicsError> {
    let event_loop = EventLoop::new().expect("Failed to setup event loop");
    let mut input = WinitInputHelper::new();
    let (mut window, mut pixels) =
        setup((width as u32, height as u32), &options, title, &event_loop)?;

    if options.confine_cursor {
        #[cfg(target_os = "macos")]
        let _ = window.set_cursor_grab(CursorGrabMode::Locked);
        #[cfg(not(target_os = "macos"))]
        let _ = window.set_cursor_grab(CursorGrabMode::Confined);
    }

    if options.hide_cursor {
        window.set_cursor_visible(false);
    }

    #[cfg(feature = "window_prefs")]
    if let Some(mut prefs) = system.window_prefs() {
        prefs.load().map_err(|e| LoadingWindowPref(e.to_string()))?;
        prefs.restore(&mut window);
    }

    let mut timing = Timing::new(options.ups);
    let mut mouse = MouseData::default();

    event_loop
        .run(move |event, target| {
            timing.update();
            match &event {
                Event::LoopExiting => {
                    system.on_window_closed();
                    #[cfg(feature = "window_prefs")]
                    if let Some(mut prefs) = system.window_prefs() {
                        prefs.store(&window);
                        //can't return from here so just print out error
                        let _ = prefs
                            .save()
                            .map_err(|err| eprintln!("Unable to save prefs: {err:?}"));
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Occluded(hidden) => system.on_visibility_changed(!hidden),
                    WindowEvent::Focused(focused) => system.on_focus_changed(*focused),
                    WindowEvent::RedrawRequested => {
                        let mut graphics = Graphics::new_u8_rgba(pixels.frame_mut(), width, height)
                            .expect("Creating graphics wrapper");
                        system.render(&mut graphics);
                        timing.renders += 1;
                        if pixels
                            .render()
                            .map_err(|e| eprintln!("pixels.render() failed: {e:?}"))
                            .is_err()
                        {
                            system.on_window_closed();
                            target.exit();
                            return;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            timing.accumulated_time += timing.delta;
            while timing.accumulated_time >= timing.fixed_time_step {
                system.update(&timing, &mut window);
                timing.accumulated_time -= timing.fixed_time_step;
                timing.updates += 1;
            }

            if input.update(&event) {
                if input.close_requested() || input.destroyed() {
                    system.on_window_closed();
                    target.exit();
                    return;
                }

                if let Some(size) = input.window_resized() {
                    pixels
                        .resize_surface(size.width, size.height)
                        .expect("Unable to resize buffer");
                }

                if let Some(mc) = input.cursor() {
                    let (x, y) = pixels
                        .window_pos_to_pixel(mc)
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    mouse.xy = coord!(x, y);
                    system.on_mouse_move(&mouse);
                }

                let mut held_buttons = vec![];
                for button in system.keys_used() {
                    if input.key_held(*button) {
                        held_buttons.push(*button);
                    }
                }
                if !held_buttons.is_empty() {
                    system.on_key_down(held_buttons);
                }

                let mut released_buttons = vec![];
                for button in system.keys_used() {
                    if input.key_released(*button) {
                        released_buttons.push(*button);
                    }
                }
                if !released_buttons.is_empty() {
                    system.on_key_up(released_buttons);
                }

                if input.mouse_pressed(MouseButton::Left) {
                    mouse.add_down(mouse.xy, MouseButton::Left);
                    system.on_mouse_down(&mouse, MouseButton::Left);
                }
                if input.mouse_pressed(MouseButton::Right) {
                    mouse.add_down(mouse.xy, MouseButton::Right);
                    system.on_mouse_down(&mouse, MouseButton::Right);
                }
                if input.mouse_pressed(MouseButton::Middle) {
                    mouse.add_down(mouse.xy, MouseButton::Middle);
                    system.on_mouse_down(&mouse, MouseButton::Middle);
                }

                if input.mouse_released(MouseButton::Left) {
                    mouse.add_up(MouseButton::Left);
                    system.on_mouse_up(&mouse, MouseButton::Left);
                }
                if input.mouse_released(MouseButton::Right) {
                    mouse.add_up(MouseButton::Right);
                    system.on_mouse_up(&mouse, MouseButton::Right);
                }
                if input.mouse_released(MouseButton::Middle) {
                    mouse.add_up(MouseButton::Middle);
                    system.on_mouse_up(&mouse, MouseButton::Middle);
                }

                let scroll = input.scroll_diff();
                if scroll.0 != 0.0 || scroll.1 != 0.0 {
                    system.on_scroll(&mouse, scroll.0.trunc() as isize, scroll.1.trunc() as isize);
                }

                window.request_redraw();
            }

            if system.should_exit() {
                target.exit();
            }

            timing.update_fps();

            timing.last = timing.now;
        })
        .expect("Error when executing event loop");

    Ok(())
}
