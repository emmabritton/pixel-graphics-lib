# Changelog

### Verion 0.20.2

- Update deps (fix tilemap bug)

### Verion 0.20.1

- Update deps (fix controller bug)

### Verion 0.20.0

### Breaking

- You **MUST** pick either the `pixels` or `softbuffer` feature now
    - Previously this was using pixels only, so set to `pixels` and everything should work the exact same
- Add support for `softbuffer`
- Remove exact dep versions
- Add `set_mouse_cursor()` for TextField
- Add `&Window` as last param on `update()` methods
- Update deps

### Version 0.19.1

- Draw submenus on the left if there's not enough room on the right
- Update deps

### Version 0.19.0

- BREAKING
    - Change scenes param from `&[KeyCode]` to `&FxHashSet<KeyCode>`
    - Remove view styles from dialog style
    - Clicking a toggle button in a group will now automatically select it and unselect the others
- Add `Checkbox` view
- Add new options to `layout!`
    - `[left|right|centerh]_to_[left|right|centerh]_of`
    - `[top|bottom|centerv]_to_[top|bottom|centerv]_of`
    - `align_centerv`, `align_centerh`
- Add `MenuBar::uncheck_all_children`
- Add feature `copypaste` which adds copying and pasting to `TextField`s

### Version 0.18.0

- BREAKING
    - Rename `UiElement` to `PixelView`
    - Rename `ElementState` to `ViewState`
- Add `MenuBar` view and `MenuBarStyle`
- Update buffer lib
- Add `id` method to `Scene`
- Allow the text field cursor to be set using the mouse
    - Also fix bug where programmatically changing the contents could leave the field in an invalid state

### Version 0.17.1

- Update buffer lib

### Version 0.17.0

- Update buffer lib
    - Add forwarding features `embedded` and `notosans`
- Add `layout!` macro to positioning views
- Fix crash when creating window bigger than monitor
- Buttons now wrap text by default

### Version 0.16.0

- Add pre and post render and update for scenes

### Version 0.15.4

- Update game utils lib

### Version 0.15.3

- Update buffer lib

### Version 0.15.2

- Change `System` to use `MouseData`
- Update buffer lib
- Add `scenes` feature

### Version 0.15.1

- Update buffer lib

### Version 0.15.0

- Add `MouseButton::Middle`
- Fix bug where `Scene::on_mouse_down` and `System::on_mouse_down` were called continuously
- Add `Scene::on_mouse_drag` and `Scene::on_mouse_click`
- Fix serde feature
- Add `mint` feature
- Change `Scene` and `UiElement` to use `MouseData` to track position and button state

### Version 0.14.3

- Update buffer lib

### Version 0.14.2

- Fix prelude/imports

### Version 0.14.1

- Add `images` feature
- Update buffer lib
- Remove shapes lib dep

### Version 0.14.0

- Add `controller_xinput` feature
- Remove `controller` from default feature

### Version 0.13.11

- Fix bug where `on_key_down` and `on_key_up` were called every loop

### Version 0.13.10

- Fix controller support

### Version 0.13.9

- Update game_util libs
- Add controller to `Scene::update` and `Scene::render` when `controller` feature is enabled
- Change `held_keys` to `&[KeyCode]`

### Version 0.13.8

- Update buffer and game_util libs

### Version 0.13.7

- Extract prefs to `simple-game-utils`
- Add function keys to `ALL`
- Make file dialogs optional
- Add version to `WindowPreferences`

### Version 0.13.6

- Update buffer and shapes libs

### Version 0.13.5

- Update buffer and shapes libs

### Version 0.13.4

- Update buffer lib

### Version 0.13.3

- Update game utils lib

### Version 0.13.2

- Extract `Timing` and `Timer` to `simple-game-utils`

### Version 0.13.1

- Expose `KeyCode` from winit

### Version 0.13.0

- Update winit to 0.29.4
    - They have redesigned the keyboard handling
        - Please see their changelog for more info
        - Most relevant is that `VirtualKeyCode` is now `KeyCode` and some of the keys have names (such as `Return`
          to `Enter`)
- Renamed `action_keys` to `keys_used` and set to all by default

### Version 0.12.1

- Update buffer lib to 0.12.0

### Version 0.12.0

- Add `column_layout` and `row_layout` macros
- Add `NAVIGATION` key set
- Add navigation to scenes keys
- Update pixels (and wgpu)

### Version 0.11.8

- Add `unfocus` and `swap_focus` macros for text fields
- Add `Sentence` and `Raw` filters for text fields

### Version 0.11.7

- Fix issue where shift was ignored for text fields

### Version 0.11.6

- Add `Timer`

### Version 0.11.5

- Change `action_keys` to return a reference

### Version 0.11.4

- Add `set_position` to `UiElement` and all views
- Add `ColumnLayout` and `RowLayout` which can position a collection of views into columns or rows

### Version 0.11.3

- Update buffer/ici-files

### Version 0.11.2

- Update buffer/ici-files

### Version 0.11.1

- Update to graphics-shapes 0.2.1
    - Adding Ellipse

### Version 0.11.0

- Update to graphics-shapes 0.2.0
    - Adding intersects and contains check
    - Removing Ellipse
- Update to buffer-graphics-lib 0.11.0
    - Adding clipping
- Add `mouse_xy` to `on_key_up`, `on_key_down` for `Scene`

### Version 0.10.8

- Use exact version of winit_input_helper to add horz scrolling
- Add horz scrolling

### Version 0.10.7

- Update buffer lib

### Version 0.10.6

- Update buffer lib

### Version 0.10.5

- Update buffer lib

### Version 0.10.4

- Update buffer lib

### Version 0.10.3

- Update buffer lib

### Version 0.10.2

- Update buffer lib

### Version 0.10.1

- Update buffer lib

### Version 0.10.0

- Swap x and y parameters on scroll
- Add `on_key_down()` and `on_key_up()` to `Scene`
- Add mouse button to `Scene` `on_mouse_down()` and `on_mouse_up()`
- Remove `on_key_press` as it's the same as `on_key_up()`
- Add more docs

### Version 0.9.3

- Update buffer lib

### Version 0.9.2

- Update pixels dep
- update API to eventually support horizontal mouse scrolling, for now x scroll will always be 0

### Version 0.9.1

- Update winit

### Version 0.9.0

- Fix OK button positioning in alerts
- Add icon buttons, toggle icon buttons, tooltips
- Add delete key to TYPING
- Add cursor movement and text scrolling to text field
- Add min and max width to text field
- Add groups for toggle buttons and toggle icon buttons
- Add disabled and error state for all UI elements
- Add style to toggle button

### Version 0.8.5

- Fix bug in load dialog UI
- Update buffer dep (adds IndexedImage)

### Version 0.8.4

- Update buffer graphics lib

### Version 0.8.3

- Add UI
- Add scenes
- Add dialogs

### Version 0.8.2

- Update buffer graphics lib
- Added option for vsync

### Version 0.8.2

- Update buffer graphics lib

### Version 0.8.1

- Update buffer graphics lib
- Update shapes lib

### Version 0.8.0

- Add prelude
- Update buffer graphics lib
- Update shapes lib
- Unlocks frame rate
- Replace `delta` in `update()` with `Timing` that provides more info
- Fix when update and render are called to achieve more consistent update/render rates
    - Use `fixed_time_step` instead of `delta` now generally

### Version 0.7.4

- Update buffer graphics lib

### Version 0.7.3

- Update buffer graphics lib

### Version 0.7.2

- Update buffer graphics lib

### Version 0.7.1

- Update buffer graphics lib

### Version 0.7.0

- Update buffer graphics lib

### Version 0.6.4

- Update buffer graphics and pixels

### Version 0.6.3

- Update buffer graphics lib (adding Ellipse)

### Version 0.6.2

- Add on_key_pressed for System

### Version 0.6.1

- Add VirtualKeyCode collections

### Version 0.6.0

- Add `run` method that handles setting up and running winit

### Version 0.5.2

- Update dev deps and examples

### Version 0.5.1

- Add methods to clear or delete window preferences
- Move buffer graphics dep to dev-deps as it's not needed any more

### Version 0.5.0

- Extract all drawing code to a separate lib [Buffer graphics](https://github.com/raybritton/buffer-graphics-lib)
    - There's a few minor difference now:
        - `setup()` now returns `(Window, Pixels)`
        - On every frame create `Graphics` and use that to draw
        - All drawing methods are the same
    - This was done so the graphics can be done without Pixels or to support double buffering, etc

### Version 0.4.0

- Update dependencies:
    - `pixels` to `0.8.0`
    - `winit` to `0.27.2`
    - `winit_input_helper` to `0.13.0`
    - `image` to `0.24.3`
- Text now uses `TextPos::Px` and `TextPos::Coord` for positioning instead of multiple methods
- You may need to run `cargo update -p raw-window-handle@0.4`
- Most of the drawing methods (`draw_rect`, `draw_circle`, etc) now
  support `isize`, `usize`, `i32`, `u32`, `i64`, `u64`, `f32` and `f64`, these are converted using `as isize`

### Version 0.3.1

- Add half() and double() methods for Point and UPoint
- Add neg() for Point
- Add set_translate, get_translate and update_translate commands for PixelWrapper

### Version 0.3.0

- BREAKING CHANGES:
    - Point renamed to UPoint
    - Vec2 renamed to Point
    - setup() now takes an amount to scale by rather than a bool
- Windows will now start with provided size and in the middle of the screen

### Version 0.2.0

- Add two features:
    - Image wrapper - this is used to load pngs, etc into `Image`s using the [Image](https://github.com/image-rs/image)
      crate
    - Window prefs - provides functions to save and restore window positions
- Add examples
- Changes `PixelWrapper` to use `isize` instead of `usize` generally

### Version 0.1.1

- Add draw_circle and draw_circle_filled

### Version 0.1.0

- Initial Release