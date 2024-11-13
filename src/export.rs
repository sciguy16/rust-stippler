use crate::{
    canvas::{Canvas, Color},
    rasterize::rasterize_circle,
    seed::Seeds,
};
use csv::Writer;
use image::{Rgb, RgbImage};
use std::error::Error;
use voronoi::Point;

pub fn save_image(path: &str, canvas: Canvas) {
    let width = canvas.pixels.len() as u32;
    let height = canvas.pixels[0].len() as u32;

    let mut img = RgbImage::new(width, height);
    let mut r;
    let mut g;
    let mut b;
    let mut color: [f32; 3];
    for x in 0..width {
        for y in 0..height {
            color = canvas.pixels[x as usize][y as usize];
            r = (color[0] * 255.0).round() as u8;
            g = (color[1] * 255.0).round() as u8;
            b = (color[2] * 255.0).round() as u8;

            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    // println!("{} exported.", path);

    img.save(path).expect("Could not save image");
}
pub fn save_rgb_image(path: &str, canvas: Canvas) {
    let width = canvas.pixels.len() as u32;
    let height = canvas.pixels[0].len() as u32;

    let mut img = RgbImage::new(width, height);
    let mut r;
    let mut g;
    let mut b;
    let mut color: [f32; 3];
    for x in 0..width {
        for y in 0..height {
            color = canvas.pixels[x as usize][y as usize];
            r = color[0].round() as u8;
            g = color[1].round() as u8;
            b = color[2].round() as u8;

            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    // println!("{} exported.", path);

    img.save(path).expect("Could not save image");
}

pub fn visualize_frame(
    frame: u16,
    seeds: &Seeds,
    width: usize,
    _height: usize,
    scale: usize,
    background_color: Color,
    _dot_color: Color,
) {
    let mut canvas = Canvas::solid_color(width * scale, width * scale, background_color);
    let mut file_name = "sequence/".to_string();
    let mut scaled_point: Point;
    let (mut x, mut y);
    for point in &seeds.coords {
        //hacky, need to fix this
        x = f64::try_from(point.x).unwrap() * scale as f64;
        y = f64::try_from(point.y).unwrap() * scale as f64;
        scaled_point = Point::new(x, y);
        rasterize_circle(&scaled_point, 4, [0.0, 0.0, 0.0], &mut canvas)
    }

    file_name.push_str(&frame.to_string());
    file_name.push_str(".jpg");
    save_image(&file_name[..], canvas);
}

pub fn export_points(path: &str, points: &Vec<Point>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    let (mut x, mut y);
    for point in points {
        x = point.x.to_string();
        y = point.y.to_string();
        wtr.write_record(&[x, y])?;
    }
    wtr.flush()?;
    Ok(())
}
