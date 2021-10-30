// NOTE: The Y axis is inverted in the image crate. At least
// it's the opposite of Nuke. In Nuke, the bottom of the image
// is 0 and the top is 1079. Inverse of that here.
// Took forever to figure out.
//
// Usage:
// - Make sure image is 1920x1080
// - Scale up and stretch until the patches hit the frame edges
// - Blur a ton to average - Working on proper averaging
// - Export a JPG or PNG
use image::{ImageBuffer, Rgb};
use std::{fmt, fs::OpenOptions, io::Write, process};
use structopt::StructOpt;

// Totally unscientific
const COLORCHECKER_CLASSIC_COORDS: [Pixel; 24] = [
    Pixel { x: 145, y: 129 },
    Pixel { x: 480, y: 129 },
    Pixel { x: 810, y: 129 },
    Pixel { x: 1135, y: 129 },
    Pixel { x: 1460, y: 129 },
    Pixel { x: 1790, y: 129 },
    Pixel { x: 145, y: 405 },
    Pixel { x: 480, y: 405 },
    Pixel { x: 810, y: 405 },
    Pixel { x: 1135, y: 405 },
    Pixel { x: 1460, y: 405 },
    Pixel { x: 1790, y: 405 },
    Pixel { x: 145, y: 679 },
    Pixel { x: 480, y: 679 },
    Pixel { x: 810, y: 679 },
    Pixel { x: 1135, y: 679 },
    Pixel { x: 1460, y: 679 },
    Pixel { x: 1790, y: 679 },
    Pixel { x: 145, y: 959 },
    Pixel { x: 480, y: 959 },
    Pixel { x: 810, y: 959 },
    Pixel { x: 1135, y: 959 },
    Pixel { x: 1460, y: 959 },
    Pixel { x: 1790, y: 959 },
];

macro_rules! attempt {
    ($x:expr, $message:expr) => {
        $x.unwrap_or_else(|_| {
            eprintln!("Error: {}", $message);
            process::exit(1);
        });
    };
    ($x:expr) => {
        $x.unwrap_or_else(|e| {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        });
    };
}

#[derive(Debug)]
struct Pixel {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Dataset {
    x: Vec<[f32; 3]>,
}

impl Dataset {
    fn new(x: Vec<[f32; 3]>) -> Self {
        Self { x }
    }

    fn from_colorchecker(image: ImageBuffer<Rgb<f32>, Vec<f32>>, coords: [Pixel; 24]) -> Self {
        let mut values = Vec::new();
        for i in coords.iter() {
            values.push(image.get_pixel(i.x, i.y).0);
        }
        Self::new(values)
    }
}

impl fmt::Display for Dataset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.x {
            writeln!(f, "{:.20} {:.20} {:.20}", i[0], i[1], i[2])?;
        }
        Ok(())
    }
}

// Totally overboard
#[derive(StructOpt)]
#[structopt(
    name = "Get ColorChecker",
    about = "Extract datasets from a colorchecker."
)]
struct Cli {
    /// Image of color checker
    filename: String,

    /// Name of file to write dataset to if desired
    #[structopt(long, short)]
    output_name: Option<String>,
}

fn main() {
    let args = Cli::from_args();

    println!("Reading image..."); // Takes long enough
    let image = attempt!(image::open(args.filename), "Can't open image").to_rgb32f();
    let dataset = Dataset::from_colorchecker(image, COLORCHECKER_CLASSIC_COORDS);

    if let Some(name) = args.output_name {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(name)
            .unwrap();
        attempt!(write!(file, "{}", dataset));
    } else {
        print!("{}", dataset);
    }
}
