mod image;
mod window;
use crate::image::write_to_file;
use crate::window::run_window;
use raytracer::run;
use structopt::StructOpt;

/// Toy raytracer.
#[derive(StructOpt)]
struct Cli {
    /// Width of the scene
    #[structopt(default_value = "200")]
    width: usize,
    /// Height of the scene
    #[structopt(default_value = "100")]
    height: usize,
    /// Display the result in a floating window
    #[structopt(short = "w", long = "window")]
    window: bool,
    /// Store the resulting image to a file
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let buffer = run(args.width, args.height);

    if let Some(file) = args.file {
        write_to_file(&file, &buffer, args.width, args.height);
    }

    if args.window {
        if let Err(e) = run_window(&buffer, args.width, args.height) {
            panic!("Whops: {:?}", e);
        }
    }
}
