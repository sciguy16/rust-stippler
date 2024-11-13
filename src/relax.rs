use crate::{
    canvas::WeightedCanvas, export::visualize_frame, geometry::UnorderedPolygon,
    rasterize::weighted_raster_centroid, seed::Seeds,
};
use display_utils::unicode_block_bar;
use std::{io::Write, path::Path};
use voronoi::{voronoi, Point};

pub fn lloyd_relax(
    start_seeds: &Seeds,
    iterations: u16,
    width: f64,
    image_path: &Path,
    frames: bool,
) -> Vec<Point> {
    let mut seeds = start_seeds.clone();
    // TO DO: figure out what condition causes weighted_raster_centroid to return NaN

    //initializing variables
    let (mut poly, mut sorted_poly);
    let mut c_r;
    let mut new_points;
    let mut faces;
    let mut vor_diagram;
    if frames {
        visualize_frame(
            0,
            &seeds,
            width as usize,
            width as usize,
            2,
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0],
        )
    }
    for i in 0..iterations {
        //create voronoi diagram
        vor_diagram = voronoi(seeds.coords, width);
        //faces of diagram
        faces = voronoi::make_polygons(&vor_diagram);
        //creating weight array (grayscale)
        let mut weights = WeightedCanvas::from_image(image_path);
        new_points = Vec::new();
        for face in faces {
            //creating unordered polygon from region
            poly = UnorderedPolygon::from_face(&face);
            // sorting ordered polygon
            sorted_poly = poly.sort();

            //creating the weighted centroid of the polygon
            c_r = weighted_raster_centroid(&sorted_poly, &mut weights);

            new_points.push(c_r);
        }
        seeds.coords = new_points;

        // --------------
        // EXPORTING SEQUENCE
        // --------------

        if frames {
            visualize_frame(
                i + 1,
                &seeds,
                width as usize,
                width as usize,
                2,
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
            )
        }

        // ----------- Simple progress bar -----------
        print!("{esc}c", esc = 27 as char);
        println!("Performing relaxation iterations:\n");
        for _ in 0..i {
            print!("{}", unicode_block_bar(1, 1.0));
        }
        for _ in 0..(iterations - i) {
            print!(" ");
            std::io::stdout().flush().unwrap();
        }
        println!("({}/{})\n", i + 1, iterations);
        std::io::stdout().flush().unwrap();
        // -------------------------------------------
    }
    seeds.coords
}
