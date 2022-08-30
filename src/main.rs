mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
use canvas::{random_color, Canvas, Weighted_Canvas, random_grayscale};
use export::save_png;
use geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use rasterize::{ rasterize_circle, scanline_rasterize_polygon, weighted_polygon_centroid};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};


fn main() {
    const WIDTH: i32 = 500;
    const HEIGHT: i32 = 500;
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const _BLACK: [f32; 3] = [0.0, 0.0, 0.0];

    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);

    let mut seeds = Seeds::uniform(&canvas, 400);
    let start_seeds = seeds.clone();
    let vor_diagram = voronoi(seeds.coords, WIDTH as f64);

    let faces = voronoi::make_polygons(&vor_diagram);
    let mut poly;
    let mut sorted_poly;
    let mut _c;
    let mut _cR;
    let mut c;
    let mut color;
    let mut gray = canvas.to_grayscale();
    for face in &faces {
        poly = Unordered_Polygon::from_face(face);
        sorted_poly = poly.sort();

        _c = vertex_centroid(&sorted_poly.vertices);
        _cR = weighted_polygon_centroid(&sorted_poly, &mut gray);
        c = Point::new(_c[0], _c[1]);
        color = random_color();
        // color = random_grayscale();
        scanline_rasterize_polygon(&sorted_poly,color, &mut canvas);
        //Skips lines -- is it the floor() function?
        //somehow the ys are duplicating/lagging
        rasterize_circle(&c, 2, _BLUE, &mut canvas);
        rasterize_circle(&_cR, 2, _GREEN, &mut canvas);
    }
    for point in &start_seeds.coords {
        rasterize_circle(point, 2, _BLACK, &mut canvas)
    }
    export::save_grayscale_png("grayscale_canvas.png", gray);
    save_png("canvas.png", canvas);
}


