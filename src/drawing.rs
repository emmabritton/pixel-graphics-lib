use crate::color::Color;
use crate::image::Image;
use crate::math::Vec2;
use crate::text::{normal_letters, small_letters, TextSize};
use pixels::Pixels;

pub struct PixelWrapper {
    pub pixels: Pixels,
    px_count: usize,
    width: usize,
    translate: Vec2,
}

impl PixelWrapper {
    pub fn new(mut pixels: Pixels, width: usize) -> Self {
        let px_count = pixels.get_frame().len();
        PixelWrapper {
            pixels,
            px_count,
            width,
            translate: Vec2::default(),
        }
    }
}

impl PixelWrapper {
    /// Convert an x,y coord to idx for use with `self.pixels`
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) * 4
    }
}

impl PixelWrapper {
    /// Copy entire pixels array to an image
    ///
    /// Although the method takes `&mut self` it doesn't mutate anything
    pub fn copy_to_image(&mut self) -> Image {
        let height = self.px_count / self.width;
        let pixels = self
            .pixels
            .get_frame()
            .chunks_exact(4)
            .map(|px| Color {
                r: px[0],
                g: px[1],
                b: px[2],
                a: px[3],
            })
            .collect::<Vec<Color>>();
        Image::new(pixels, self.width, height)
    }

    /// Get top left pixel coord for letter coord
    pub fn get_px_for_char(&self, x: usize, y: usize, size: TextSize) -> (usize, usize) {
        let (width, height) = size.get_size();
        let margin = size.get_margin();
        (x * (width + margin), y * (height + margin))
    }

    /// Get width and height for string
    pub fn get_text_size(&self, text: &str, width: usize, size: TextSize) -> (usize, usize) {
        let len = text.chars().count();
        let x = if len < width { len } else { width };
        let y = (len as f64 / width as f64).ceil() as usize;
        let (width, height) = size.get_size();
        let margin = size.get_margin();
        ((width + margin) * x, (height + margin) * y)
    }
}

impl PixelWrapper {
    /// Sets every pixel to the same color, this ignores translate
    pub fn clear(&mut self, color: Color) {
        self.pixels.get_frame().chunks_exact_mut(4).for_each(|px| {
            px[0] = color.r;
            px[1] = color.g;
            px[2] = color.b;
        });
    }

    /// Draw an image at `x`, `y`
    pub fn draw_image(&mut self, start_x: usize, start_y: usize, image: &Image) {
        let mut x = 0;
        for (y, row) in image.pixels.chunks_exact(image.width()).enumerate() {
            for px in row {
                if px.a > 0 {
                    self.set_pixel(start_x + x, start_y + y, *px);
                }
                x += 1;
            }
            x = 0;
        }
    }

    /// Draw a letter at a letter coord
    /// XY valid from 0,0 to 47,25 for small
    ///               0,0 to 23,12 for normal
    pub fn draw_letter(&mut self, x: usize, y: usize, chr: char, size: TextSize, color: Color) {
        if chr == ' ' {
            return;
        }
        let (width, height) = size.get_size();
        let margin = size.get_margin();
        self.draw_letter_px(
            x * (width + margin),
            y * (height + margin),
            chr,
            size,
            color,
        )
    }

    /// Draw a letter at pixel coord
    pub fn draw_letter_px(&mut self, x: usize, y: usize, chr: char, size: TextSize, color: Color) {
        if chr == ' ' {
            return;
        }
        let (width, height) = size.get_size();
        let px: Vec<bool> = match size {
            TextSize::Small => small_letters::get_px(chr).to_vec(),
            TextSize::Normal => normal_letters::get_px(chr).to_vec(),
        };
        let start_x = x;
        let start_y = y;
        for x in 0..width {
            for y in 0..height {
                let i = x + y * width;
                if px[i] {
                    self.set_pixel(x + start_x, y + start_y, color);
                }
            }
        }
    }

    /// Draws text in lines at most `width` chars long at pixel coord
    ///
    /// Width must be max chars - x
    /// max chars is 47 for small
    ///              23 for normal
    /// So if x was 5 and size was small then max width is 42
    pub fn draw_text_px(
        &mut self,
        text: &str,
        line_width: usize,
        mut x: usize,
        mut y: usize,
        size: TextSize,
        color: Color,
    ) {
        let start_x = x;
        for char in text.chars() {
            self.draw_letter_px(x, y, char, size, color);
            x += size.get_size().0 + size.get_margin();
            if x >= line_width * (size.get_size().0 + size.get_margin()) + start_x {
                y += size.get_size().1 + size.get_margin();
                x = start_x;
            }
        }
    }

    /// Draws text in lines at most `width` chars long at letter coord
    ///
    /// Width must be max chars - x
    /// max chars is 47 for small
    ///              23 for normal
    /// So if x was 5 and size was small then max width is 42
    pub fn draw_text(
        &mut self,
        text: &str,
        line_width: usize,
        mut x: usize,
        mut y: usize,
        size: TextSize,
        color: Color,
    ) {
        let start_x = x;
        for char in text.chars() {
            self.draw_letter(x, y, char, size, color);
            x += 1;
            if x >= start_x + line_width {
                y += 1;
                x = start_x;
            }
        }
    }

    /// Get the RGB values for a pixel
    /// Alpha will always be 255
    ///
    /// If `use_translate` is true than the x,y will be updated with `self.translate`
    ///
    /// Although the method takes `&mut self` it doesn't mutate anything
    #[inline]
    pub fn get_pixel(&mut self, x: usize, y: usize, use_translate: bool) -> Option<Color> {
        let (x, y) = if use_translate {
            (x as isize + self.translate.x, y as isize + self.translate.y)
        } else {
            (x as isize, y as isize)
        };

        if x >= 0 && y >= 0 && x < self.width as isize {
            let idx = self.index(x as usize, y as usize);
            if idx < self.px_count {
                return Some(Color::rgb(
                    self.pixels.get_frame()[idx],
                    self.pixels.get_frame()[idx + 1],
                    self.pixels.get_frame()[idx + 2],
                ));
            }
        }

        None
    }

    /// Set the RGB values for a pixel
    /// This ignores alpha, so 0,0,0,0 will draw a black pixel
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let x = x as isize + self.translate.x;
        let y = y as isize + self.translate.y;
        if x >= 0 && y >= 0 && x < self.width as isize {
            let idx = self.index(x as usize, y as usize);

            if idx < self.px_count {
                self.pixels.get_frame()[idx] = color.r;
                self.pixels.get_frame()[idx + 1] = color.g;
                self.pixels.get_frame()[idx + 2] = color.b;
            }
        }
    }

    /// Draw a filled rectangle from `x1,y1` to `x2,y2` in `color`
    pub fn draw_rect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: Color) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.set_pixel(x, y, color);
            }
        }
    }

    /// Draw a hollow rectangle from `x1,y1` to `x2,y2` in `color`
    pub fn draw_frame(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: Color) {
        self.draw_line(x1, y1, x1, y2, color);
        self.draw_line(x1, y1, x2, y1, color);
        self.draw_line(x1, y2, x2, y2, color);
        self.draw_line(x2, y1, x2, y2, color);
    }

    /// Draw line from `x1,y1` to `x2,y2` in `color`
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: Color) {
        let mut delta = 0;
        let x1 = x1 as isize;
        let y1 = y1 as isize;
        let x2 = x2 as isize;
        let y2 = y2 as isize;
        let dx = isize::abs(x2 - x1);
        let dy = isize::abs(y2 - y1);
        let dx2 = dx * 2;
        let dy2 = dy * 2;
        let ix: isize = if x1 < x2 { 1 } else { -1 };
        let iy: isize = if y1 < y2 { 1 } else { -1 };
        let mut x = x1;
        let mut y = y1;
        if dx >= dy {
            loop {
                self.set_pixel(x as usize, y as usize, color);
                if x == x2 {
                    break;
                }
                x += ix;
                delta += dy2;
                if delta > dx {
                    y += iy;
                    delta -= dx2;
                }
            }
        } else {
            loop {
                self.set_pixel(x as usize, y as usize, color);
                if y == y2 {
                    break;
                }
                y += iy;
                delta += dx2;
                if delta > dy {
                    x += ix;
                    delta -= dy2;
                }
            }
        }
    }
}
