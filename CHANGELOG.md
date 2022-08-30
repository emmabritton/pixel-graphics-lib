# Changelog

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