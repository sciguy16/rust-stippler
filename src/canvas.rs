use image::{io::Reader as ImageReader, GenericImageView};
use std::path::Path;

pub type Color = [f32; 3];
pub struct Canvas {
    pub pixels: Vec<Vec<Color>>,
}
pub struct WeightedCanvas {
    //grayscale
    pub pixel_weights: Vec<Vec<f32>>,
}

impl WeightedCanvas {
    pub fn read_pixel(&self, x: usize, y: usize) -> f32 {
        if x < self.pixel_weights[0].len() && y < self.pixel_weights.len() {
            self.pixel_weights[x][y]
        } else {
            0.0 //return black
        }
    }

    pub fn from_image(path: &Path) -> Self {
        let img = ImageReader::open(path)
            .expect("Error.")
            .decode()
            .expect("Error.");
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut pixels = vec![vec![0.0; width]; height];
        for pixel in img.pixels() {
            let r = pixel.2[0] as f32;
            let g = pixel.2[1] as f32;
            let b = pixel.2[2] as f32;
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;

            pixels[x][y] = (r + g + b) / (3.0 * 255.0);
        }
        Self {
            pixel_weights: pixels,
        }
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![[0.0, 0.0, 0.0]; height]; width],
        }
    }
    pub fn solid_color(width: usize, height: usize, color: Color) -> Self {
        Self {
            pixels: vec![vec![color; height]; width],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.pixels[0].len() && y < self.pixels.len() {
            self.pixels[x][y] = color
        }
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> Color {
        if x < self.pixels[0].len() && y < self.pixels.len() {
            self.pixels[x][y]
        } else {
            [0.0, 0.0, 0.0] //return black
        }
    }
    pub fn from_image(path: &Path) -> Self {
        let img = ImageReader::open(path)
            .expect("Error.")
            .decode()
            .expect("Error.");
        let width = img.width() as usize;
        let height = img.height() as usize;
        let (mut r, mut g, mut b);
        let (mut x, mut y);
        let mut pixels = vec![vec![[0.0; 3]; width]; height];
        for pixel in img.pixels() {
            r = pixel.2[0] as f32;
            g = pixel.2[1] as f32;
            b = pixel.2[2] as f32;
            x = pixel.0 as usize;
            y = pixel.1 as usize;
            pixels[x][y] = [r, g, b];
        }
        Self { pixels }
    }
}
