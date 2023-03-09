use crate::prelude::Positioning::LeftTop;
use crate::prelude::*;
use crate::ui::dir_panel::FileEntry::*;
use crate::ui::prelude::Positioning::Center;
use crate::ui::prelude::*;
use std::cmp::Ordering;
use std::fs::{read_dir, ReadDir};
use std::path::PathBuf;

const ENTRY_FORMAT: TextFormat =
    TextFormat::new(WrappingStrategy::Ellipsis(35), Small, BLACK, LeftTop);
const ERROR_FORMAT: TextFormat =
    TextFormat::new(WrappingStrategy::SpaceBeforeCol(20), Normal, RED, Center);

#[derive(Debug, PartialEq, Clone, Eq)]
enum FileEntry {
    ParentDir(String),
    File(FileInfo),
    Dir(String, String),
}

impl FileEntry {
    pub fn to_result(&self) -> DirResult {
        match self {
            ParentDir(path) => DirResult::new(path.clone(), false),
            File(info) => DirResult::new(info.path.clone(), true),
            Dir(path, _) => DirResult::new(path.clone(), false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirResult {
    pub path: String,
    pub is_file: bool,
}

impl DirResult {
    pub fn new(path: String, is_file: bool) -> Self {
        Self { path, is_file }
    }
}

impl Ord for FileEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for FileEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = if let ParentDir(_) = self {
            Ordering::Less
        } else if let ParentDir(_) = other {
            Ordering::Greater
        } else {
            match (self, other) {
                (File(info), Dir(_, name)) => info.filename.cmp(name),
                (Dir(_, name), File(info)) => name.cmp(&info.filename),
                (Dir(_, lhs), Dir(_, rhs)) => lhs.cmp(rhs),
                (File(lhs), File(rhs)) => lhs.filename.cmp(&rhs.filename),
                (_, _) => Ordering::Equal,
            }
        };
        Some(result)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FileInfo {
    pub path: String,
    pub filename: String,
    pub size: String,
}

/// Call [get_click_result] if [on_mouse_click] returns true
#[derive(Debug)]
pub struct DirPanel {
    current_dir: String,
    files: Vec<FileEntry>,
    first_visible_file_index: usize,
    entry_visible_count: usize,
    background: ShapeCollection,
    bounds: Rect,
    error: Option<String>,
    highlight: Option<usize>,
    allowed_ext: Option<String>,
    state: ElementState,
}

impl DirPanel {
    pub fn new(current_dir: &str, bounds: Rect, allowed_ext: Option<&str>) -> Self {
        let mut background = ShapeCollection::new();
        InsertShape::insert_above(&mut background, bounds.clone(), fill(WHITE));
        InsertShape::insert_above(&mut background, bounds.clone(), stroke(DARK_GRAY));
        let entry_visible_count = bounds.height() / (Small.get_size().1 + Small.get_spacing());
        let mut panel = Self {
            error: None,
            current_dir: current_dir.to_string(),
            bounds,
            files: vec![],
            entry_visible_count,
            first_visible_file_index: 0,
            background,
            highlight: None,
            allowed_ext: allowed_ext.map(|s| s.to_string()),
            state: ElementState::Normal,
        };
        panel.set_dir(current_dir);
        panel
    }
}

fn fs_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes}B")
    } else if bytes < 1024 * 1024 {
        format!("{}KB", bytes / 1024)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{}MB", bytes / 1024 / 1024)
    } else {
        format!("{}GB", bytes / 1024 / 1024 / 1024)
    }
}

fn get_files(path: &str, dir: ReadDir, allowed_ext: &Option<String>) -> Vec<FileEntry> {
    let parent = PathBuf::from(path)
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let mut results = vec![ParentDir(parent)];
    for file in dir.flatten() {
        if let Ok(file_type) = file.file_type() {
            if file_type.is_file() {
                let include = if let Some(allowed) = allowed_ext {
                    &file
                        .path()
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                        == allowed
                } else {
                    true
                };
                if include {
                    results.push(File(FileInfo {
                        path: file.path().to_string_lossy().to_string(),
                        filename: file.file_name().to_string_lossy().to_string(),
                        size: fs_size(file.metadata().unwrap().len()),
                    }))
                }
            } else if file_type.is_dir() {
                results.push(Dir(
                    file.path().to_string_lossy().to_string(),
                    file.file_name().to_string_lossy().to_string(),
                ))
            }
        }
    }
    results
}

impl DirPanel {
    pub fn set_dir(&mut self, path: &str) {
        self.error = None;
        self.first_visible_file_index = 0;
        match read_dir(path) {
            Ok(dir) => {
                let mut files = get_files(path, dir, &self.allowed_ext);
                files.sort();
                self.files = files;
            }
            Err(err) => self.error = Some(err.to_string()),
        }
    }

    #[must_use]
    pub fn highlighted(&self) -> Option<DirResult> {
        if let Some(i) = self.highlight {
            self.files.get(i).map(|e| e.to_result())
        } else {
            None
        }
    }

    pub fn set_highlight(&mut self, path: &str) {
        for (i, entry) in self.files.iter().enumerate() {
            let entry_path = match entry {
                ParentDir(path) => path,
                File(info) => &info.path,
                Dir(path, _) => path,
            };
            if path == entry_path {
                self.highlight = Some(i);
                break;
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn current_dir(&self) -> &str {
        &self.current_dir
    }

    pub fn on_scroll(&mut self, diff: isize) {
        let factor = diff.abs() % 5;
        let up = diff < 0;
        if up && self.first_visible_file_index > 0 {
            self.first_visible_file_index = self
                .first_visible_file_index
                .saturating_sub(factor.unsigned_abs());
        }
        if !up && (self.first_visible_file_index + self.entry_visible_count < self.files.len()) {
            self.first_visible_file_index = (self.first_visible_file_index + factor.unsigned_abs())
                .min(self.files.len() - self.entry_visible_count);
        }
    }

    fn bounds_for_row(&self, row: usize) -> Rect {
        let xy = self.bounds.top_left()
            + (
                2,
                row * (Small.get_spacing() + Small.get_size().1) + Small.get_spacing() * 2,
            );
        Rect::new(
            xy,
            (
                self.bounds.right() - 2,
                xy.y + (Small.get_size().1) as isize,
            ),
        )
    }

    pub fn on_mouse_click(&mut self, mouse_xy: Coord) -> Option<DirResult> {
        if self.state == ElementState::Disabled {
            return None;
        }
        if self.bounds.contains(mouse_xy) {
            for i in 0..self.entry_visible_count {
                if self.bounds_for_row(i).contains(mouse_xy) {
                    return self
                        .files
                        .get(i + self.first_visible_file_index)
                        .map(|e| e.to_result());
                }
            }
        }
        None
    }
}

impl UiElement for DirPanel {
    #[inline]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        graphics.draw(&self.background);

        if let Some(txt) = &self.error {
            graphics.draw_text(txt, TextPos::px(self.bounds.center()), ERROR_FORMAT);
        } else {
            let mut row = 0;
            for i in self.first_visible_file_index
                ..self.first_visible_file_index + self.entry_visible_count
            {
                let highlighted = self.highlight.map(|r| r == i).unwrap_or_default();
                if i < self.files.len() {
                    let back = self.bounds_for_row(row);
                    if back.contains(mouse_xy) || highlighted {
                        graphics.draw_rect(
                            back.clone(),
                            fill(if highlighted { CYAN } else { LIGHT_GRAY }),
                        );
                    }
                    match &self.files[i] {
                        ParentDir(_) => {
                            graphics.draw_text("..", TextPos::px(back.top_left()), ENTRY_FORMAT)
                        }
                        File(info) => graphics.draw_text(
                            &info.filename,
                            TextPos::px(back.top_left()),
                            ENTRY_FORMAT,
                        ),
                        Dir(_, name) => {
                            graphics.draw_text(name, TextPos::px(back.top_left()), ENTRY_FORMAT)
                        }
                    }
                    row += 1;
                }
            }
        }
    }

    fn update(&mut self, _: &Timing) {}

    #[inline]
    fn set_state(&mut self, new_state: ElementState) {
        self.state = new_state;
    }

    #[inline]
    fn get_state(&self) -> ElementState {
        self.state
    }
}
