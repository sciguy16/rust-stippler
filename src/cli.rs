use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[clap(help = "Input file")]
    pub input: PathBuf,

    #[clap(
        short = 'n',
        long = "point-count",
        default_value = "1000",
        help = "number of points"
    )]
    pub count: usize,

    #[clap(
        short = 'i',
        long = "iterations",
        default_value = "60",
        help = "number of iterations"
    )]
    pub iterations: u16,

    #[clap(
        short = 't',
        long = "threshold",
        default_value = "0.5",
        help = "grayscale threshold"
    )]
    pub threshold: f32,

    #[clap(short = 'f', long = "frames", help = "export frames")]
    pub frames: bool,

    #[clap(
        short = 'c',
        long = "cartesian",
        default_value = "50",
        help = "use uniform seed spacing"
    )]
    pub cartesian_spacing: u32,

    #[clap(
        short = 'm',
        long = "mosaic",
        help = "export colored voronoi mosaic of final image"
    )]
    pub save_mosaic: bool,
}
