use clap::Parser;
use std::path::PathBuf;

// // ARGUMENTS //
// let args: Vec<String> = env::args().collect();
// let image_path = &args[1];
// let points: usize = args[2].to_string().parse().unwrap();
// let iterations: u16 = args[3].to_string().parse().unwrap();
// let threshold: f32 = args[4].to_string().parse().unwrap();
// // --------- //

#[derive(Debug, Parser)]
#[clap(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Input file
    // #[clap(parse(from_os_str))]
    pub input: PathBuf,

    /// Set point count
    #[clap(short = 'n', long = "point-count", default_value = "1000")]
    pub count: usize,

    // Set number of iterations
    #[clap(short = 'i', long = "iterations", default_value = "60")]
    pub iterations: u16,

    // Set threshold
    #[clap(short = 't', long = "threshold", default_value = "0.5")]
    pub threshold: f32,

    // Save iteration images
    #[clap(short = 'f', long = "frames")]
    pub frames: bool,

    // Specify cartesian sample spacing
    #[clap(short = 'c', long = "cartesian", default_value = "50")]
    pub cartesian_spacing: u32,

    // Export mosaic image
    #[clap(short = 'm', long = "mosaic")]
    pub save_mosaic: bool,
}

pub fn print_help() {
    println!("USAGE:");
    println!("    ./stippling -- [weight_image]");

    println!("OPTIONS:");
    println!("    -n, --point-count,      number of points");
    println!("    -c, --cartesian,        use uniform seed spacing");
    println!("    -i, --iterations,       number of iterations");
    println!("    -t, --threshold,        grayscale threshold");
    println!("    -f, --frames,           export frames");
    println!("    -m, --mosaic,           export colored voronoi mosaic of final image");
    println!("    -h, --help,             show help");
}
