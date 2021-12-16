use crate::color::{Color, WHITE};
use crate::scaling::*;
use crate::Tint;

/// Images are rectangles of pixels that be manipulated and drawn on screen
///
/// Transparency is accounted for when drawing images
#[derive(Clone)]
pub struct Image {
    pub(crate) pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    /// Create a image of width x height size using provided pixels
    pub fn new(pixels: Vec<Color>, width: usize, height: usize) -> Self {
        assert_eq!(
            pixels.len(),
            width * height,
            "Invalid pixel array length, expected: {}, found: {}",
            width * height,
            pixels.len()
        );
        Image {
            pixels,
            width,
            height,
        }
    }

    /// Create a white image of width x height size
    pub fn new_blank(width: usize, height: usize) -> Self {
        let pixels = vec![WHITE; width * height];
        Image::new(pixels, width, height)
    }
}

impl Image {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, value: Color) {
        self.pixels[y * self.width + x] = value;
    }

    /// Flip image horizontally
    pub fn flip_horizontal(&mut self) {
        let half_width = (self.width as f32 / 2.).floor() as usize;
        for y in 0..self.height {
            for x in 0..half_width {
                let y = y * self.width;
                unsafe {
                    std::ptr::swap(
                        &mut self.pixels[y + x],
                        &mut self.pixels[y + self.width - 1 - x],
                    );
                }
            }
        }
    }

    /// Flip image vertically
    pub fn flip_vertical(&mut self) {
        let half_height = (self.height as f32 / 2.).floor() as usize;
        for y in 0..half_height {
            unsafe {
                std::ptr::swap_nonoverlapping(
                    &mut self.pixels[y * self.width],
                    &mut self.pixels[(self.height - 1 - y) * self.width],
                    self.width,
                );
            }
        }
    }

    /// Return a new image after scaling
    pub fn scale(&self, algo: Scaling) -> Image {
        match algo {
            Scaling::NearestNeighbour { x_scale, y_scale } => {
                scale_nearest_neighbor(self, usize::from(x_scale), usize::from(y_scale))
            }
            Scaling::Epx2x => scale_epx(self),
            Scaling::Epx4x => scale_epx(&scale_epx(self)),
        }
    }
}

impl Tint for Image {
    fn tint_add(&mut self, r_diff: isize, g_diff: isize, b_diff: isize, a_diff: isize) {
        for pixel in self.pixels.iter_mut() {
            (*pixel).tint_add(r_diff, g_diff, b_diff, a_diff);
        }
    }

    fn tint_mul(&mut self, r_diff: f32, g_diff: f32, b_diff: f32, a_diff: f32) {
        for pixel in self.pixels.iter_mut() {
            (*pixel).tint_mul(r_diff, g_diff, b_diff, a_diff);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;
    use crate::image::Image;
    use crate::Tint;

    fn make_image() -> Image {
        Image::new(
            vec![
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(7),
                Color::gray(8),
                Color::gray(9),
            ],
            3,
            3,
        )
    }

    #[test]
    fn constructor() {
        let image = make_image();

        assert_eq!(image.width, 3);
        assert_eq!(image.height, 3);
        assert_eq!(image.pixels.len(), 9);
        assert_eq!(
            image.pixels,
            vec![
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(7),
                Color::gray(8),
                Color::gray(9)
            ]
        );
    }

    #[test]
    fn test_flip() {
        let image = make_image();
        assert_eq!(
            image.pixels,
            vec![
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(7),
                Color::gray(8),
                Color::gray(9)
            ]
        );
        let mut horz = make_image();
        horz.flip_horizontal();
        assert_eq!(
            horz.pixels,
            vec![
                Color::gray(3),
                Color::gray(2),
                Color::gray(1),
                Color::gray(6),
                Color::gray(5),
                Color::gray(4),
                Color::gray(9),
                Color::gray(8),
                Color::gray(7)
            ]
        );
        let mut vert = make_image();
        vert.flip_vertical();
        assert_eq!(
            vert.pixels,
            vec![
                Color::gray(7),
                Color::gray(8),
                Color::gray(9),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(1),
                Color::gray(2),
                Color::gray(3)
            ]
        );
        let mut horz_vert = make_image();
        horz_vert.flip_horizontal();
        horz_vert.flip_vertical();
        assert_eq!(
            horz_vert.pixels,
            vec![
                Color::gray(9),
                Color::gray(8),
                Color::gray(7),
                Color::gray(6),
                Color::gray(5),
                Color::gray(4),
                Color::gray(3),
                Color::gray(2),
                Color::gray(1)
            ]
        );
        let mut vert_horz = make_image();
        vert_horz.flip_horizontal();
        vert_horz.flip_vertical();
        assert_eq!(
            vert_horz.pixels,
            vec![
                Color::gray(9),
                Color::gray(8),
                Color::gray(7),
                Color::gray(6),
                Color::gray(5),
                Color::gray(4),
                Color::gray(3),
                Color::gray(2),
                Color::gray(1)
            ]
        );
    }

    #[test]
    fn tint_add() {
        let mut image = make_image();
        image.tint_add(10, 20, 30, -50);
        assert_eq!(
            image.pixels,
            vec![
                Color {
                    r: 11,
                    g: 21,
                    b: 31,
                    a: 205
                },
                Color {
                    r: 12,
                    g: 22,
                    b: 32,
                    a: 205
                },
                Color {
                    r: 13,
                    g: 23,
                    b: 33,
                    a: 205
                },
                Color {
                    r: 14,
                    g: 24,
                    b: 34,
                    a: 205
                },
                Color {
                    r: 15,
                    g: 25,
                    b: 35,
                    a: 205
                },
                Color {
                    r: 16,
                    g: 26,
                    b: 36,
                    a: 205
                },
                Color {
                    r: 17,
                    g: 27,
                    b: 37,
                    a: 205
                },
                Color {
                    r: 18,
                    g: 28,
                    b: 38,
                    a: 205
                },
                Color {
                    r: 19,
                    g: 29,
                    b: 39,
                    a: 205
                },
            ]
        );
    }

    #[test]
    fn tint_mul() {
        let mut image = make_image();
        image.tint_mul(0.5, 1.0, 2.0, 1.0);
        assert_eq!(
            image.pixels,
            vec![
                Color {
                    r: 1,
                    g: 1,
                    b: 2,
                    a: 255
                },
                Color {
                    r: 1,
                    g: 2,
                    b: 4,
                    a: 255
                },
                Color {
                    r: 2,
                    g: 3,
                    b: 6,
                    a: 255
                },
                Color {
                    r: 2,
                    g: 4,
                    b: 8,
                    a: 255
                },
                Color {
                    r: 3,
                    g: 5,
                    b: 10,
                    a: 255
                },
                Color {
                    r: 3,
                    g: 6,
                    b: 12,
                    a: 255
                },
                Color {
                    r: 4,
                    g: 7,
                    b: 14,
                    a: 255
                },
                Color {
                    r: 4,
                    g: 8,
                    b: 16,
                    a: 255
                },
                Color {
                    r: 5,
                    g: 9,
                    b: 18,
                    a: 255
                },
            ]
        );
    }
}
