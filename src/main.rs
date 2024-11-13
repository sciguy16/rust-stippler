mod canvas;
mod cli;
mod export;
mod geometry;
mod rasterize;
mod relax;
mod seed;

use canvas::{Canvas, WeightedCanvas};
use clap::Parser;
use cli::Opt;
use seed::Seeds;
use voronoi::voronoi;

fn main() {
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const _BLACK: [f32; 3] = [0.0, 0.0, 0.0];

    // ARGUMENTS //

    let opt = Opt::parse();
    let points: usize = opt.count;
    let iterations: u16 = opt.iterations;
    let threshold: f32 = opt.threshold;
    let save_frames: bool = opt.frames;
    let save_mosaic: bool = opt.save_mosaic;

    // --------- //
    //weight canvas
    let mut weight_canvas = WeightedCanvas::from_image(&opt.input);
    let width = weight_canvas.pixel_weights[0].len();
    let height = weight_canvas.pixel_weights.len();

    //main canvas1
    let mut canvas2 = Canvas::solid_color(width, height, _WHITE);
    let mut canvas3 = Canvas::solid_color(width, height, _WHITE);
    let mut color_canvas = Canvas::new(width, height);

    //creating start seeds
    // let mut seeds = Seeds::uniform(&canvas2, points);
    // let mut seeds = Seeds::cartesian(&weight_canvas,cartesian_spacing as f64, threshold);
    let seeds = Seeds::rejection_sample(&weight_canvas, points, threshold);
    // let seeds = Seeds::pdf_rejection_sample(&weight_canvas,points, threshold);

    let relaxed = relax::lloyd_relax(&seeds, iterations, width as f64, &opt.input, save_frames);

    export::export_points("start_points.csv", &seeds.coords).expect("Failed to save csv.");
    for seed in seeds.coords {
        rasterize::rasterize_circle(&seed, 2, _BLACK, &mut canvas2)
    }
    export::export_points("end_points.csv", &relaxed).expect("Failed to save csv.");
    for seed in &relaxed {
        rasterize::rasterize_circle(seed, 3, _BLACK, &mut canvas3)
    }

    let vor_diagram = voronoi(relaxed, width as f64);

    let faces = voronoi::make_polygons(&vor_diagram);

    println!("Saved images:");
    export::save_image("start_seeds.jpg", canvas2);
    println!("\tstart_seeds.jpg");

    export::save_image("end_seeds.jpg", canvas3);
    println!("\tend_seeds.jpg");

    if save_mosaic {
        rasterize::color_sampled_voronoi(&opt.input, faces, &mut color_canvas, &mut weight_canvas);
        export::save_rgb_image("mosaic.png", color_canvas);
        println!("\tmosaic.jpg");
    }
}
