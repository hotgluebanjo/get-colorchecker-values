// NOTE: The Y axis is inverted in the image crate. At least
// it's the opposite of Nuke. In Nuke, the bottom of the image
// is 0 and the top is 1079. Inverse of that here.
// Took forever to figure out.
use image::{ImageBuffer, Rgb};
use std::{fmt, fs::OpenOptions, io::Write, process};
use structopt::StructOpt;

// Totally unscientific
const COLORCHECKER_CLASSIC_COORDS: [Pixel; 24] = [
    Pixel { x: 169, y: 122 },
    Pixel { x: 547, y: 122 },
    Pixel { x: 823, y: 122 },
    Pixel { x: 1100, y: 122 },
    Pixel { x: 1377, y: 122 },
    Pixel { x: 1654, y: 122 },
    Pixel { x: 169, y: 399 },
    Pixel { x: 547, y: 399 },
    Pixel { x: 823, y: 399 },
    Pixel { x: 1100, y: 399 },
    Pixel { x: 1377, y: 399 },
    Pixel { x: 1654, y: 399 },
    Pixel { x: 169, y: 677 },
    Pixel { x: 547, y: 677 },
    Pixel { x: 823, y: 677 },
    Pixel { x: 1100, y: 677 },
    Pixel { x: 1377, y: 677 },
    Pixel { x: 1654, y: 677 },
    Pixel { x: 169, y: 957 },
    Pixel { x: 547, y: 957 },
    Pixel { x: 823, y: 957 },
    Pixel { x: 1100, y: 957 },
    Pixel { x: 1377, y: 957 },
    Pixel { x: 1654, y: 957 },
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
    about = "Extract datasets from a colorchecker.",
    version = "0.1.0"
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
