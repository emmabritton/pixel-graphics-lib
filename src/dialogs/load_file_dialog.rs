use crate::dialogs::{dialog_background, FileDialogResults};
use crate::scenes::*;
use crate::ui::prelude::TextFilter::*;
use crate::ui::prelude::*;
use crate::*;
use buffer_graphics_lib::prelude::*;
use directories::UserDirs;
use std::fmt::Debug;

/// You should use something like `rfd` instead of this
#[derive(Debug)]
pub struct LoadFileDialog<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq>
where
    SR: FileDialogResults<SR>,
{
    result: SceneUpdateResult<SR, SN>,
    dir_panel: DirPanel,
    cancel: Button,
    open: Button,
    background: ShapeCollection,
    current_dir_field: TextField,
    downloads: Button,
    docs: Button,
    home: Button,
    load: Button,
}

impl<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq> LoadFileDialog<SR, SN>
where
    SR: FileDialogResults<SR>,
{
    pub fn new(
        allowed_ext: Option<&str>,
        width: usize,
        height: usize,
        style: &DialogStyle,
    ) -> Box<Self> {
        let background = dialog_background(width, height, style);
        //This is a potential problem
        let path = UserDirs::new()
            .unwrap()
            .download_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let dir_panel = DirPanel::new(
            &path,
            Rect::new(
                style.bounds.top_left() + (6, 40),
                style.bounds.top_left() + (style.bounds.width() - 4, 140),
            ),
            allowed_ext,
        );
        let open = Button::new(
            style.bounds.top_left() + (140, 146),
            "Open",
            Some(50),
            &style.button,
        );
        let cancel = Button::new(
            style.bounds.top_left() + (6, 146),
            "Cancel",
            None,
            &style.button,
        );
        let current_dir = TextField::new(
            style.bounds.top_left() + (6, 6),
            37,
            PixelFont::Standard4x5,
            (Some(dir_panel.bounds().width()), None),
            &path,
            &[All],
            &style.text_field,
        );
        let docs = Button::new(
            style.bounds.top_left() + (6, 18),
            "Docs",
            None,
            &style.button,
        );
        let downloads = Button::new(
            style.bounds.top_left() + (43, 18),
            "Downloads",
            None,
            &style.button,
        );
        let home = Button::new(
            style.bounds.top_left() + (122, 18),
            "Home",
            None,
            &style.button,
        );
        let load = Button::new(
            style.bounds.top_left() + (158, 18),
            "Load",
            None,
            &style.button,
        );
        Box::new(Self {
            result: SceneUpdateResult::Nothing,
            dir_panel,
            cancel,
            open,
            background,
            current_dir_field: current_dir,
            downloads,
            docs,
            home,
            load,
        })
    }
}

impl<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq> LoadFileDialog<SR, SN>
where
    SR: FileDialogResults<SR>,
{
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData) {
        graphics.draw(&self.background);
        self.current_dir_field.render(graphics, mouse);
        self.dir_panel.render(graphics, mouse);
        self.open.render(graphics, mouse);
        self.cancel.render(graphics, mouse);
        self.docs.render(graphics, mouse);
        self.downloads.render(graphics, mouse);
        self.load.render(graphics, mouse);
        self.home.render(graphics, mouse);
    }

    fn update(&mut self, timing: &Timing) -> SceneUpdateResult<SR, SN> {
        self.current_dir_field.update(timing);
        self.result.clone()
    }
}

impl<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq> Scene<SR, SN>
    for LoadFileDialog<SR, SN>
where
    SR: FileDialogResults<SR>,
{
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn render(
        &self,
        graphics: &mut Graphics,
        mouse: &MouseData,
        _: &[KeyCode],
        _: &GameController,
    ) {
        self.render(graphics, mouse)
    }

    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &[KeyCode]) {
        self.render(graphics, mouse)
    }

    fn on_key_up(&mut self, key: KeyCode, _: &MouseData, held_keys: &[KeyCode]) {
        if self.current_dir_field.is_focused() {
            if key == KeyCode::KeyV {
                if held_keys.contains(&&KeyCode::ControlRight) {}
            } else if key == KeyCode::Enter {
            } else {
                self.current_dir_field.on_key_press(key, held_keys);
            }
        }
    }

    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        button: MouseButton,
        _: &[KeyCode],
    ) {
        if button != MouseButton::Left {
            return;
        }
        if self.downloads.on_mouse_click(down_at, mouse.xy) {
            self.current_dir_field.set_content(
                &UserDirs::new()
                    .unwrap()
                    .download_dir()
                    .unwrap()
                    .to_string_lossy(),
            );
            self.dir_panel.set_dir(self.current_dir_field.content());
        }
        if self.docs.on_mouse_click(down_at, mouse.xy) {
            self.current_dir_field.set_content(
                &UserDirs::new()
                    .unwrap()
                    .document_dir()
                    .unwrap()
                    .to_string_lossy(),
            );
            self.dir_panel.set_dir(self.current_dir_field.content());
        }
        if self.home.on_mouse_click(down_at, mouse.xy) {
            self.current_dir_field
                .set_content(&UserDirs::new().unwrap().home_dir().to_string_lossy());
            self.dir_panel.set_dir(self.current_dir_field.content());
        }
        if self.cancel.on_mouse_click(down_at, mouse.xy) {
            self.result = SceneUpdateResult::Pop(None);
        }
        if self.load.on_mouse_click(down_at, mouse.xy) {
            self.dir_panel.set_dir(self.current_dir_field.content());
        }
        self.current_dir_field.on_mouse_click(down_at, mouse.xy);
        if let Some(result) = self.dir_panel.on_mouse_click(down_at, mouse.xy) {
            if result.is_file {
                self.dir_panel.set_highlight(&result.path);
            } else {
                self.current_dir_field.set_content(&result.path);
                self.dir_panel.set_dir(self.current_dir_field.content());
            }
        }
        if self.open.on_mouse_click(down_at, mouse.xy) {
            if let Some(entry) = self.dir_panel.highlighted() {
                self.result = SceneUpdateResult::Pop(Some(SR::load_file_result(entry.path)))
            }
        }
    }

    fn on_scroll(&mut self, mouse: &MouseData, _: isize, y_diff: isize, _: &[KeyCode]) {
        self.dir_panel.on_scroll(mouse.xy, y_diff);
    }

    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn update(
        &mut self,
        timing: &Timing,
        _: &MouseData,
        _: &[KeyCode],
        _: &GameController,
    ) -> SceneUpdateResult<SR, SN> {
        self.update(timing)
    }

    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn update(
        &mut self,
        timing: &Timing,
        _: &MouseData,
        _: &[KeyCode],
    ) -> SceneUpdateResult<SR, SN> {
        self.update(timing)
    }

    fn resuming(&mut self, _: Option<SR>) {}

    fn is_dialog(&self) -> bool {
        true
    }
}
