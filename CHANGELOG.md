# Changelog

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
- Most of the drawing methods (`draw_rect`, `draw_circle`, etc) now support `isize`, `usize`, `i32`, `u32`, `i64`, `u64`, `f32` and `f64`, these are converted using `as isize`

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
    - Image wrapper - this is used to load pngs, etc into `Image`s using the [Image](https://github.com/image-rs/image) crate
    - Window prefs - provides functions to save and restore window positions
- Add examples
- Changes `PixelWrapper` to use `isize` instead of `usize` generally

### Version 0.1.1

- Add draw_circle and draw_circle_filled

### Version 0.1.0

- Initial Release