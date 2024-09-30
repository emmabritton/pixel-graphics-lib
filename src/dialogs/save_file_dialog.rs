use crate::prelude::*;
use crate::scenes::*;
use crate::ui::prelude::TextFilter::{All, Filename};
use crate::ui::prelude::*;
use buffer_graphics_lib::prelude::*;
use directories::UserDirs;
use std::fmt::Debug;
use std::path::PathBuf;
use winit::window::Window;

/// You should use something like `rfd` instead of this
#[derive(Debug)]
pub struct SaveFileDialog<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq>
where
    SR: FileDialogResults<SR>,
{
    result: SceneUpdateResult<SR, SN>,
    current_dir_field: TextField,
    dir_panel: DirPanel,
    name_field: TextField,
    downloads: Button,
    docs: Button,
    home: Button,
    load: Button,
    save: Button,
    cancel: Button,
    background: ShapeCollection,
    expected_ext: Option<String>,
}

impl<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq> SaveFileDialog<SR, SN>
where
    SR: FileDialogResults<SR>,
{
    pub fn new(
        filepath: Option<String>,
        expected_ext: Option<&str>,
        width: usize,
        height: usize,
        style: &UiStyle,
    ) -> Box<Self> {
        let background = dialog_background(width, height, &style.dialog);
        let path = match filepath {
            None => UserDirs::new()
                .unwrap()
                .download_dir()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            Some(path) => PathBuf::from(path)
                .parent()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        };

        let dir_panel = DirPanel::new(
            &path,
            Rect::new(
                style.dialog.bounds.top_left() + (6, 40),
                style.dialog.bounds.top_left() + (style.dialog.bounds.width() - 4, 120),
            ),
            expected_ext,
        );
        let save = Button::new(
            style.dialog.bounds.top_left() + (140, 146),
            "Save",
            Some(50),
            &style.button,
        );
        let cancel = Button::new(
            style.dialog.bounds.top_left() + (6, 146),
            "Cancel",
            None,
            &style.button,
        );
        let current_dir = TextField::new(
            style.dialog.bounds.top_left() + (6, 6),
            37,
            PixelFont::Standard4x5,
            (Some(dir_panel.bounds().width()), None),
            &path,
            &[All],
            &style.text_field,
        );
        let name_field = TextField::new(
            style.dialog.bounds.top_left() + (6, 121),
            26,
            PixelFont::Standard6x7,
            (Some(dir_panel.bounds().width()), None),
            "",
            &[Filename],
            &style.text_field,
        );
        let docs = Button::new(
            style.dialog.bounds.top_left() + (6, 18),
            "Docs",
            None,
            &style.button,
        );
        let downloads = Button::new(
            style.dialog.bounds.top_left() + (43, 18),
            "Downloads",
            None,
            &style.button,
        );
        let home = Button::new(
            style.dialog.bounds.top_left() + (122, 18),
            "Home",
            None,
            &style.button,
        );
        let load = Button::new(
            style.dialog.bounds.top_left() + (158, 18),
            "Load",
            None,
            &style.button,
        );
        Box::new(Self {
            result: SceneUpdateResult::Nothing,
            current_dir_field: current_dir,
            dir_panel,
            name_field,
            downloads,
            docs,
            home,
            load,
            save,
            cancel,
            background,
            expected_ext: expected_ext.map(|s| s.to_string()),
        })
    }
}

impl<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq> SaveFileDialog<SR, SN>
where
    SR: FileDialogResults<SR>,
{
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData) {
        self.background.render(graphics);
        self.name_field.render(graphics, mouse);
        self.current_dir_field.render(graphics, mouse);
        self.dir_panel.render(graphics, mouse);
        self.save.render(graphics, mouse);
        self.home.render(graphics, mouse);
        self.downloads.render(graphics, mouse);
        self.docs.render(graphics, mouse);
        self.home.render(graphics, mouse);
        self.load.render(graphics, mouse);
        self.cancel.render(graphics, mouse);
    }

    fn update(&mut self, timing: &Timing) -> SceneUpdateResult<SR, SN> {
        self.name_field.update(timing);
        self.current_dir_field.update(timing);
        self.result.clone()
    }
}

impl<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq> Scene<SR, SN>
    for SaveFileDialog<SR, SN>
where
    SR: FileDialogResults<SR>,
{
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn render(
        &self,
        graphics: &mut Graphics,
        mouse: &MouseData,
        _: &FxHashSet<KeyCode>,
        _: &GameController,
    ) {
        self.render(graphics, mouse)
    }

    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &FxHashSet<KeyCode>) {
        self.render(graphics, mouse)
    }

    fn on_key_up(&mut self, key: KeyCode, _: &MouseData, held_keys: &FxHashSet<KeyCode>) {
        self.name_field.on_key_press(key, held_keys);
        self.current_dir_field.on_key_press(key, held_keys);
    }

    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        button: MouseButton,
        _: &FxHashSet<KeyCode>,
    ) {
        if button != MouseButton::Left {
            return;
        }
        if self.cancel.on_mouse_click(down_at, mouse.xy) {
            self.result = SceneUpdateResult::Pop(None);
        }
        self.current_dir_field.on_mouse_click(down_at, mouse.xy);
        self.name_field.on_mouse_click(down_at, mouse.xy);
        if let Some(result) = self.dir_panel.on_mouse_click(down_at, mouse.xy) {
            if result.is_file {
                let filename = PathBuf::from(result.path)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                self.name_field.set_content(&filename);
            } else {
                self.dir_panel.set_dir(&result.path);
                self.current_dir_field.set_content(&result.path);
            }
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
        if self.save.on_mouse_click(down_at, mouse.xy) && !self.name_field.content().is_empty() {
            let mut path = PathBuf::from(self.current_dir_field.content());
            path.push(self.name_field.content());
            if let Some(ext) = &self.expected_ext {
                if !path.ends_with(ext) {
                    path.set_extension(ext);
                }
            }
            self.result = SceneUpdateResult::Pop(Some(SR::save_file_result(
                path.to_string_lossy().to_string(),
            )));
        }
    }

    fn on_scroll(&mut self, mouse: &MouseData, _: isize, y_diff: isize, _: &FxHashSet<KeyCode>) {
        self.dir_panel.on_scroll(mouse.xy, y_diff);
    }

    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn update(
        &mut self,
        timing: &Timing,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
        _: &GameController,
        _: &Window,
    ) -> SceneUpdateResult<SR, SN> {
        self.update(timing)
    }

    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn update(
        &mut self,
        timing: &Timing,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
        _: &Window,
    ) -> SceneUpdateResult<SR, SN> {
        self.update(timing)
    }

    fn resuming(&mut self, _: Option<SR>) {}

    fn is_dialog(&self) -> bool {
        true
    }
}
