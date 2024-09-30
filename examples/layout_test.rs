use anyhow::Result;
use buffer_graphics_lib::prelude::*;
use pixels_graphics_lib::prelude::SceneUpdateResult::*;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::ui::prelude::TextFilter::Numbers;
use pixels_graphics_lib::ui::prelude::*;
use pixels_graphics_lib::ui::text_field::TextFilter::All;
use winit::keyboard::KeyCode;

const WIDTH: usize = 280;
const HEIGHT: usize = 240;

fn main() -> Result<()> {
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, _, _| {};
    let options = Options::default();
    let test = LayoutTest::new(&options.style);
    run_scenes(
        WIDTH,
        HEIGHT,
        "Layout Tester",
        None,
        switcher,
        test,
        options,
        empty_pre_post(),
    )?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum SceneName {}

#[derive(Debug, Clone, PartialEq)]
enum SceneResult {}

pub struct LayoutTest {
    row_layout: RowLayout,
    column_layout: ColumnLayout,
    button: Button,
    icon_button: IconButton,
    toggle_button: ToggleButton,
    toggle_icon_button: ToggleIconButton,
    text_field: TextField,
    column: Button,
    row: Button,
    spacing: TextField,
    padding: TextField,
    lefttop: Button,
    center: Button,
    rightbottom: Button,
}

impl LayoutTest {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let button = Button::new((50, 50), "Button", None, &style.button);
        let toggle_button = ToggleButton::new((50, 50), "Button", None, &style.toggle_button);
        let icon_button = IconButton::new(
            (50, 50),
            "Icon",
            Positioning::RightBottom,
            IndexedImage::from_file_contents(include_bytes!("resources/icon.ici"))
                .unwrap()
                .0,
            &style.icon_button,
        );
        let toggle_icon_button = ToggleIconButton::new(
            (50, 50),
            "Toggle Icon",
            Positioning::Center,
            IndexedImage::from_file_contents(include_bytes!("resources/icon.ici"))
                .unwrap()
                .0,
            &style.toggle_icon_button,
        );
        let mut column = Button::new((0, 0), "Column", None, &style.button);
        let mut row = Button::new((0, 0), "Row", None, &style.button);
        let mut lefttop = Button::new((0, 0), "Left/Top", None, &style.button);
        let mut center = Button::new((0, 0), "Center", None, &style.button);
        let mut rightbottom = Button::new((0, 0), "Right/Bottom", None, &style.button);
        let text_field = TextField::new(
            (50, 50),
            10,
            PixelFont::Standard6x7,
            (None, None),
            "",
            &[All],
            &style.text_field,
        );
        let mut spacing = TextField::new(
            (0, 0),
            4,
            PixelFont::Standard6x7,
            (None, None),
            "0",
            &[Numbers],
            &style.text_field,
        );
        let mut padding = TextField::new(
            (0, 0),
            4,
            PixelFont::Standard6x7,
            (None, None),
            "0",
            &[Numbers],
            &style.text_field,
        );
        let row_layout = RowLayout::new_bounded(Rect::new_with_size((8, 50), 100, 100));
        let column_layout = ColumnLayout::new_bounded(Rect::new_with_size((8, 50), 100, 100));

        let temp_layout = RowLayout::new(
            8,
            8,
            Rect::new_with_size((0, 6), WIDTH, 20),
            RowGravity::Center,
        );
        temp_layout.layout(&mut [&mut row, &mut column, &mut spacing, &mut padding]);

        let temp_layout = RowLayout::new(
            8,
            8,
            Rect::new_with_size((0, 20), WIDTH, 30),
            RowGravity::Center,
        );
        temp_layout.layout(&mut [&mut lefttop, &mut center, &mut rightbottom]);

        Box::new(Self {
            row_layout,
            column_layout,
            button,
            icon_button,
            toggle_button,
            toggle_icon_button,
            text_field,
            column,
            row,
            spacing,
            padding,
            lefttop,
            center,
            rightbottom,
        })
    }
}

impl Scene<SceneResult, SceneName> for LayoutTest {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(BLUE);
        self.button.render(graphics, mouse);
        self.toggle_button.render(graphics, mouse);
        self.icon_button.render(graphics, mouse);
        self.toggle_icon_button.render(graphics, mouse);
        self.text_field.render(graphics, mouse);
        self.spacing.render(graphics, mouse);
        self.padding.render(graphics, mouse);
        self.row.render(graphics, mouse);
        self.column.render(graphics, mouse);
        self.lefttop.render(graphics, mouse);
        self.center.render(graphics, mouse);
        self.rightbottom.render(graphics, mouse);
    }

    fn on_key_up(&mut self, key: KeyCode, _: &MouseData, held: &FxHashSet<KeyCode>) {
        self.text_field.on_key_press(key, held);
        self.padding.on_key_press(key, held);
        self.spacing.on_key_press(key, held);
    }

    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        _: MouseButton,
        _: &FxHashSet<KeyCode>,
    ) {
        self.text_field.on_mouse_click(down_at, mouse.xy);
        self.spacing.on_mouse_click(down_at, mouse.xy);
        self.padding.on_mouse_click(down_at, mouse.xy);
        if self.row.on_mouse_click(down_at, mouse.xy) {
            self.row_layout.padding = self.padding.content().parse::<usize>().unwrap_or_default();
            self.row_layout.spacing = self.spacing.content().parse::<usize>().unwrap_or_default();
            self.row_layout.layout(&mut [
                &mut self.button,
                &mut self.toggle_button,
                &mut self.text_field,
                &mut self.icon_button,
                &mut self.toggle_icon_button,
            ]);
        }
        if self.column.on_mouse_click(down_at, mouse.xy) {
            self.column_layout.padding =
                self.padding.content().parse::<usize>().unwrap_or_default();
            self.column_layout.spacing =
                self.spacing.content().parse::<usize>().unwrap_or_default();
            self.column_layout.layout(&mut [
                &mut self.button,
                &mut self.toggle_button,
                &mut self.text_field,
                &mut self.icon_button,
                &mut self.toggle_icon_button,
            ]);
        }
        if self.lefttop.on_mouse_click(down_at, mouse.xy) {
            self.column_layout.gravity = ColumnGravity::Left;
            self.row_layout.gravity = RowGravity::Top;
        }
        if self.center.on_mouse_click(down_at, mouse.xy) {
            self.column_layout.gravity = ColumnGravity::Center;
            self.row_layout.gravity = RowGravity::Center;
        }
        if self.rightbottom.on_mouse_click(down_at, mouse.xy) {
            self.column_layout.gravity = ColumnGravity::Right;
            self.row_layout.gravity = RowGravity::Bottom;
        }
    }

    fn update(
        &mut self,
        timing: &Timing,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
        _: &Window,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        self.text_field.update(timing);
        self.spacing.update(timing);
        self.padding.update(timing);
        Nothing
    }

    fn resuming(&mut self, _: Option<SceneResult>) {}
}
