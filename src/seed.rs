use crate::WeightedCanvas;
use rand::Rng;
use voronoi::Point;

#[derive(Debug, Clone)]
pub struct Seeds {
    pub coords: Vec<Point>,
}

impl Seeds {
    pub fn rejection_sample(weights: &WeightedCanvas, count: usize, threshold: f32) -> Self {
        let width = weights.pixel_weights[0].len();
        let height = weights.pixel_weights.len();
        let mut seeds: Vec<Point> = Vec::new();
        let mut x: f64;
        let mut y: f64;
        let mut rng = rand::thread_rng();
        let mut i = 0;
        let mut point;
        let mut sampled_value;

        while i < count {
            x = rng.gen::<f64>() * width as f64;
            y = rng.gen::<f64>() * height as f64;
            point = Point::new(x, y);
            sampled_value = weights.read_pixel(point.x.round() as usize, point.y.round() as usize);
            if sampled_value < threshold {
                seeds.push(point);
                i += 1;
            }
        }
        Seeds { coords: seeds }
    }
}
