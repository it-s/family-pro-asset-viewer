use clap::Parser;
use std::env;

mod animation;
mod fbk;
mod file;
mod fx4;
mod hmg;
mod pal;
mod viewer;

use olc_pixel_game_engine as olc;
use viewer::Viewer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// FX4 file name
    #[arg(short, long, required = true)]
    file: String,

    /// Palette file
    #[arg(short, long)]
    pal: String,

    /// Window width
    #[arg(long, default_value_t = 640)]
    width: i32,

    /// Window height
    #[arg(long, default_value_t = 480)]
    height: i32,

    /// Initial zoom
    #[arg(long, default_value_t = 2)]
    zoom: i32,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();

    // Check file extension and stop application if not supported
    match file::get_file_type(&args.file) {
        file::FileType::Unknown => {
            println!(
                "Error: File {} has an unknown extension. Exiting.",
                &args.file
            );
            std::process::exit(1);
        }
        file::FileType::HMG | file::FileType::FBK | file::FileType::FX4 => {
            println!("Game file: {}", &args.file);
            println!("PAL file: {}", &args.pal);

            let mut viewer = Viewer::new(args.file, args.pal, args.zoom);
            olc::start(
                "Family Pro. Asset viewer",
                &mut viewer,
                args.width,
                args.height,
                1,
                1,
            )
            .unwrap();
        }
        file::FileType::PAL => {
            println!("Error: File {} is a palette file. Exiting.", &args.file);
            std::process::exit(1);
        }
    }
}
