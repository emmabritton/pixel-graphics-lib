use crate::prelude::*;
use crate::scenes::*;
use crate::ui::prelude::TextFilter::{All, Filename};
use crate::ui::prelude::*;
use buffer_graphics_lib::prelude::*;
use directories::UserDirs;
use std::fmt::Debug;
use std::path::PathBuf;

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
        style: &DialogStyle,
    ) -> Box<Self> {
        let background = dialog_background(width, height, style);
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
                style.bounds.top_left() + (6, 40),
                style.bounds.top_left() + (style.bounds.width() - 4, 120),
            ),
            expected_ext,
        );
        let save = Button::new(
            style.bounds.top_left() + (140, 146),
            "Save",
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
            Small,
            (Some(dir_panel.bounds().width()), None),
            &path,
            &[All],
            &style.text_field,
        );
        let name_field = TextField::new(
            style.bounds.top_left() + (6, 121),
            26,
            Normal,
            (Some(dir_panel.bounds().width()), None),
            "",
            &[Filename],
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

impl<SR: Clone + Debug + PartialEq, SN: Clone + Debug + PartialEq> Scene<SR, SN>
    for SaveFileDialog<SR, SN>
where
    SR: FileDialogResults<SR>,
{
    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        self.background.render(graphics);
        self.name_field.render(graphics, mouse_xy);
        self.current_dir_field.render(graphics, mouse_xy);
        self.dir_panel.render(graphics, mouse_xy);
        self.save.render(graphics, mouse_xy);
        self.home.render(graphics, mouse_xy);
        self.downloads.render(graphics, mouse_xy);
        self.docs.render(graphics, mouse_xy);
        self.home.render(graphics, mouse_xy);
        self.load.render(graphics, mouse_xy);
        self.cancel.render(graphics, mouse_xy);
    }

    fn on_key_up(&mut self, key: VirtualKeyCode, _: Coord, _: &Vec<&VirtualKeyCode>) {
        self.name_field.on_key_press(key);
        self.current_dir_field.on_key_press(key);
    }

    fn on_mouse_up(&mut self, xy: Coord, button: MouseButton, _: &Vec<&VirtualKeyCode>) {
        if button != MouseButton::Left {
            return;
        }
        if self.cancel.on_mouse_click(xy) {
            self.result = SceneUpdateResult::Pop(None);
        }
        self.current_dir_field.on_mouse_click(xy);
        self.name_field.on_mouse_click(xy);
        if let Some(result) = self.dir_panel.on_mouse_click(xy) {
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
        if self.downloads.on_mouse_click(xy) {
            self.current_dir_field.set_content(
                &UserDirs::new()
                    .unwrap()
                    .download_dir()
                    .unwrap()
                    .to_string_lossy(),
            );
            self.dir_panel.set_dir(self.current_dir_field.content());
        }
        if self.docs.on_mouse_click(xy) {
            self.current_dir_field.set_content(
                &UserDirs::new()
                    .unwrap()
                    .document_dir()
                    .unwrap()
                    .to_string_lossy(),
            );
            self.dir_panel.set_dir(self.current_dir_field.content());
        }
        if self.home.on_mouse_click(xy) {
            self.current_dir_field
                .set_content(&UserDirs::new().unwrap().home_dir().to_string_lossy());
            self.dir_panel.set_dir(self.current_dir_field.content());
        }
        if self.save.on_mouse_click(xy) && !self.name_field.content().is_empty() {
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

    fn on_scroll(&mut self, xy: Coord, _: isize, y_diff: isize, _: &Vec<&VirtualKeyCode>) {
        self.dir_panel.on_scroll(xy, y_diff);
    }

    fn update(
        &mut self,
        timing: &Timing,
        _: Coord,
        _: &Vec<&VirtualKeyCode>,
    ) -> SceneUpdateResult<SR, SN> {
        self.name_field.update(timing);
        self.current_dir_field.update(timing);
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SR>) {}

    fn is_dialog(&self) -> bool {
        true
    }
}
