use crate::prelude::winit;
use crate::GraphicsError::LoadingWindowPref;
use crate::{GraphicsError, MouseData, Options, System, WindowScaling};
use log::error;
use simple_game_utils::prelude::Timing;
use std::marker::PhantomData;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{CursorGrabMode, Window, WindowId};

pub(crate) fn make_window(
    event_loop: &ActiveEventLoop,
    system: &mut Box<dyn System>,
    options: &Options,
    width: usize,
    height: usize,
    title: String,
) -> Result<(f64, Rc<Window>), GraphicsError> {
    let mut attr = Window::default_attributes();
    attr.title = title;

    let mut window: Window = event_loop
        .create_window(attr)
        .expect("Window created after resuming");
    let mut factor = match options.scaling {
        WindowScaling::Native => window.scale_factor(),
        WindowScaling::Double => window.scale_factor() + 2.0,
        WindowScaling::Quad => window.scale_factor() + 4.0,
    };
    let px_size: PhysicalSize<u32> =
        LogicalSize::new(width as u32, height as u32).to_physical(factor);

    window.set_min_inner_size(Some(px_size));
    let _ = window.request_inner_size(px_size);
    window.set_visible(true);

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
        if let Err(e) = prefs.load().map_err(|e| LoadingWindowPref(e.to_string())) {
            error!("Unable to restore window size/pos: {e:?}");
        }
        prefs.restore(&mut window);
    }
    if window.inner_size() != px_size {
        let horz_scale = window.inner_size().width as usize / width;
        let vert_scale = window.inner_size().height as usize / height;
        let new_scale: usize = horz_scale.min(vert_scale);
        factor = new_scale as f64;
    }
    Ok((factor, Rc::new(window)))
}

///
/// Taken from https://raw.githubusercontent.com/rust-windowing/softbuffer/refs/heads/master/examples/utils/winit_app.rs
///

#[allow(unused_mut)]
pub(crate) fn run_app(
    event_loop: EventLoop<()>,
    mut app: impl ApplicationHandler<()> + 'static,
) -> Result<(), EventLoopError> {
    event_loop.run_app(&mut app)
}

pub(crate) struct WinitApp<T, Init, Handler> {
    init: Init,

    event: Handler,

    state: Option<T>,

    system: Box<dyn System>,
    options: Options,
    timing: Timing,
    mouse: MouseData,
}

pub(crate) struct WinitAppBuilder<T, Init> {
    init: Init,
    _marker: PhantomData<Option<T>>,
    system: Box<dyn System>,
    options: Options,
}

impl<T, Init> WinitAppBuilder<T, Init>
where
    Init: FnMut(&ActiveEventLoop, &mut Box<dyn System>, &Options) -> T,
{
    pub fn new(system: Box<dyn System>, options: Options, init: Init) -> Self {
        Self {
            init,
            system,
            options,
            _marker: PhantomData,
        }
    }

    pub fn setup<F>(self, handler: F) -> WinitApp<T, Init, F>
    where
        F: FnMut(
            &mut T,
            Event<()>,
            &ActiveEventLoop,
            &mut Box<dyn System>,
            &mut Timing,
            &mut MouseData,
            &Options,
        ),
    {
        WinitApp::new(self.init, handler, self.system, self.options)
    }
}

impl<T, Init, Handler> WinitApp<T, Init, Handler>
where
    Init: FnMut(&ActiveEventLoop, &mut Box<dyn System>, &Options) -> T,
    Handler: FnMut(
        &mut T,
        Event<()>,
        &ActiveEventLoop,
        &mut Box<dyn System>,
        &mut Timing,
        &mut MouseData,
        &Options,
    ),
{
    pub(crate) fn new(
        init: Init,
        event: Handler,
        system: Box<dyn System>,
        options: Options,
    ) -> Self {
        Self {
            init,
            event,
            system,
            timing: Timing::new(options.ups),
            mouse: MouseData::default(),
            options,
            state: None,
        }
    }
}

impl<T, Init, Handler> ApplicationHandler for WinitApp<T, Init, Handler>
where
    Init: FnMut(&ActiveEventLoop, &mut Box<dyn System>, &Options) -> T,
    Handler: FnMut(
        &mut T,
        Event<()>,
        &ActiveEventLoop,
        &mut Box<dyn System>,
        &mut Timing,
        &mut MouseData,
        &Options,
    ),
{
    fn resumed(&mut self, el: &ActiveEventLoop) {
        debug_assert!(self.state.is_none());
        self.state = Some((self.init)(el, &mut self.system, &self.options));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        (self.event)(
            state,
            Event::WindowEvent { window_id, event },
            event_loop,
            &mut self.system,
            &mut self.timing,
            &mut self.mouse,
            &self.options,
        );
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_mut() {
            (self.event)(
                state,
                Event::AboutToWait,
                event_loop,
                &mut self.system,
                &mut self.timing,
                &mut self.mouse,
                &self.options,
            );
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let state = self.state.take();
        debug_assert!(state.is_some());
        drop(state);
    }
}
