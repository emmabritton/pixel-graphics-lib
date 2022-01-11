use crate::color::Color;
use crate::image::Image;
use crate::text::{normal_letters, small_letters, TextSize};
use pixels::Pixels;
use crate::math::Point;

pub struct PixelWrapper {
    pub pixels: Pixels,
    px_count: usize,
    width: usize,
    height: usize,
    translate: Point,
}

impl PixelWrapper {
    pub fn new(mut pixels: Pixels, width: usize, height: usize) -> Self {
        let px_count = pixels.get_frame().len();
        PixelWrapper {
            pixels,
            px_count,
            width,
            height,
            translate: Point::default(),
        }
    }
}

impl PixelWrapper {
    /// Convert an x,y coord to idx for use with `self.pixels`
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) * 4
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl PixelWrapper {
    /// Copy entire pixels array to an image
    ///
    /// Although the method takes `&mut self` it doesn't mutate anything
    pub fn copy_to_image(&mut self) -> Image {
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
        Image::new(pixels, self.width, self.height).expect("This can't fail")
    }

    /// Get top left pixel coord for letter px coord
    pub fn get_px_for_char(&self, x: usize, y: usize, size: TextSize) -> (usize, usize) {
        let (width, height) = size.get_size();
        let margin = size.get_margin();
        (x * (width + margin), y * (height + margin))
    }

    /// Get width and height for string
    ///
    /// # Arguments
    /// * `text` - The string to be measured
    /// * `width` - The line width in characters
    /// * `size` - The text size to use when measuring
    ///
    /// # Returns
    ///
    /// The width and height of the string in pixels
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
    pub fn draw_image(&mut self, start_x: isize, start_y: isize, image: &Image) {
        let mut x = 0;
        for (y, row) in image.pixels.chunks_exact(image.width()).enumerate() {
            for px in row {
                if px.a > 0 {
                    self.update_pixel(start_x + x, start_y + y as isize, *px);
                }
                x += 1;
            }
            x = 0;
        }
    }

    /// Draw a letter at a letter coord
    /// See [TextSize::get_max_characters] for maximum x and y
    pub fn draw_letter(&mut self, x: isize, y: isize, chr: char, size: TextSize, color: Color) {
        if chr == ' ' {
            return;
        }
        let (width, height) = size.get_size();
        let margin = size.get_margin() as isize;
        self.draw_letter_px(
            x * (width as isize + margin),
            y * (height as isize + margin),
            chr,
            size,
            color,
        )
    }

    /// Draw a letter at pixel coord
    pub fn draw_letter_px(&mut self, x: isize, y: isize, chr: char, size: TextSize, color: Color) {
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
                    self.update_pixel(x as isize + start_x, y as isize + start_y, color);
                }
            }
        }
    }

    /// Draws text in lines at most `width` chars long at pixel coord
    ///
    /// Width should be max chars - x
    /// See [TextSize::get_max_characters] for maximum chars
    pub fn draw_text_px(
        &mut self,
        text: &str,
        line_width: usize,
        mut x: isize,
        mut y: isize,
        size: TextSize,
        color: Color,
    ) {
        let start_x = x;
        for char in text.chars() {
            self.draw_letter_px(x, y, char, size, color);
            x += (size.get_size().0 + size.get_margin()) as isize;
            if x >= (line_width * (size.get_size().0 + size.get_margin())) as isize + start_x {
                y += (size.get_size().1 + size.get_margin()) as isize;
                x = start_x;
            }
        }
    }

    /// Draws text in lines at most `width` chars long at letter coord
    ///
    /// Width must be max chars - x
    /// See [TextSize::get_max_characters] for maximum x and y
    pub fn draw_text(
        &mut self,
        text: &str,
        line_width: usize,
        mut x: isize,
        mut y: isize,
        size: TextSize,
        color: Color,
    ) {
        let line_width = line_width as isize;
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
    pub fn get_pixel(&mut self, x: isize, y: isize, use_translate: bool) -> Option<Color> {
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

    /// Update a pixel color, using [PixelWrapper::set_pixel] or [PixelWrapper::blend_pixel] depending
    /// on whether `color`s alpha is 255 or not
    #[inline]
    pub fn update_pixel(&mut self, x: isize, y: isize, color: Color) {
        if color.a == 255 {
            self.set_pixel(x, y, color);
        } else {
            self.blend_pixel(x, y, color);
        }
    }

    /// Set the RGB values for a pixel by blending it with the provided color
    /// This method uses alpha blending, note that the canvas pixels always have 255 alpha
    #[inline]
    pub fn blend_pixel(&mut self, x: isize, y: isize, color: Color) {
        let x = x as isize + self.translate.x;
        let y = y as isize + self.translate.y;
        if x >= 0 && y >= 0 && x < self.width as isize {
            if let Some(base) = self.get_pixel(x, y, false) {
                let new_color = base.blend(color);
                let idx = self.index(x as usize, y as usize);
                self.pixels.get_frame()[idx] = new_color.r;
                self.pixels.get_frame()[idx + 1] = new_color.g;
                self.pixels.get_frame()[idx + 2] = new_color.b;
            }
        }
    }

    /// Set the RGB values for a pixel
    /// This ignores alpha, so 255,0,0,0 will draw a red pixel
    #[inline]
    pub fn set_pixel(&mut self, x: isize, y: isize, color: Color) {
        let x = x + self.translate.x;
        let y = y + self.translate.y;
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
    pub fn draw_rect(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.update_pixel(x, y, color);
            }
        }
    }

    /// Draw a hollow rectangle from `x1,y1` to `x2,y2` in `color`
    pub fn draw_frame(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color) {
        self.draw_line(x1, y1, x1, y2, color);
        self.draw_line(x1, y1, x2, y1, color);
        self.draw_line(x1, y2, x2, y2, color);
        self.draw_line(x2, y1, x2, y2, color);
    }

    pub fn draw_circle(&mut self, x: isize, y: isize, radius: isize, color: Color) {
        let cx = x as isize;
        let cy = y as isize;
        let mut d = (5_isize - (radius as isize) * 4) / 4;
        let mut x = 0;
        let mut y = radius as isize;
        let w = self.width as isize;
        let h = self.height as isize;

        let clamp_w = |num: isize| num.clamp(0, w);
        let clamp_h = |num: isize| num.clamp(0, h);

        while x <= y {
            self.update_pixel(clamp_w(cx + x), clamp_h(cy + y), color);
            self.update_pixel(clamp_w(cx + x), clamp_h(cy - y), color);
            self.update_pixel(clamp_w(cx - x), clamp_h(cy + y), color);
            self.update_pixel(clamp_w(cx - x), clamp_h(cy - y), color);
            self.update_pixel(clamp_w(cx + y), clamp_h(cy + x), color);
            self.update_pixel(clamp_w(cx + y), clamp_h(cy - x), color);
            self.update_pixel(clamp_w(cx - y), clamp_h(cy + x), color);
            self.update_pixel(clamp_w(cx - y), clamp_h(cy - x), color);
            if d < 0 {
                d += 2 * x + 1
            } else {
                d += 2 * (x - y) + 1;
                y -= 1;
            }
            x += 1;
        }
    }

    pub fn draw_circle_filled(&mut self, x: isize, y: isize, radius: isize, color: Color) {
        let cx = x as isize;
        let cy = y as isize;
        let w = self.width as isize;
        let h = self.height as isize;
        let double_radius = (radius * radius) as isize;
        let clamp_w = |num: isize| num.clamp(0, w);
        let clamp_h = |num: isize| num.clamp(0, h);
        for y in 0..radius {
            let y = y as isize;
            let up = cy - y;
            let down = cy + y;
            let half_width = (((double_radius - y * y) as f64).sqrt().round() as isize).max(0);
            for x in 0..half_width {
                let left = cx - x;
                let right = cx + x;
                self.update_pixel(clamp_w(left), clamp_h(up), color);
                self.update_pixel(clamp_w(right), clamp_h(up), color);
                self.update_pixel(clamp_w(left), clamp_h(down), color);
                self.update_pixel(clamp_w(right), clamp_h(down), color);
            }
        }
    }

    /// Draw line from `x1,y1` to `x2,y2` in `color`
    pub fn draw_line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color) {
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
                self.update_pixel(x, y, color);
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
                self.update_pixel(x, y, color);
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
