// NOTE: The Y axis is inverted in the image crate. At least
// it's the opposite of Nuke. In Nuke, the bottom of the image
// is 0 and the top is 1079. Inverse of that here.
// Took forever to figure out.
//
// Usage:
// - Make sure image is 1920x1080
// - Scale up and stretch until the patches hit the frame edges
// - Export a JPG or PNG
use image::{ImageBuffer, Rgb};
use std::{fmt, fs::OpenOptions, io::Write, process};
use structopt::StructOpt;

const USAGE_INSTRUCTIONS: &str = "Extract datasets from a ColorChecker.
Notes:
This is a very simple app, so it's a little finicky. Here's how to prep images for it:
- Make sure the image provided is 1920x1080
- Scale up and stretch until the patches hit the frame edges
- Export a JPG or PNG";

// Totally unscientific
const COLORCHECKER_CLASSIC_COORDS: [Point2d; 24] = [
    Point2d { x: 145, y: 129 },
    Point2d { x: 480, y: 129 },
    Point2d { x: 810, y: 129 },
    Point2d { x: 1135, y: 129 },
    Point2d { x: 1460, y: 129 },
    Point2d { x: 1790, y: 129 },
    Point2d { x: 145, y: 405 },
    Point2d { x: 480, y: 405 },
    Point2d { x: 810, y: 405 },
    Point2d { x: 1135, y: 405 },
    Point2d { x: 1460, y: 405 },
    Point2d { x: 1790, y: 405 },
    Point2d { x: 145, y: 679 },
    Point2d { x: 480, y: 679 },
    Point2d { x: 810, y: 679 },
    Point2d { x: 1135, y: 679 },
    Point2d { x: 1460, y: 679 },
    Point2d { x: 1790, y: 679 },
    Point2d { x: 145, y: 959 },
    Point2d { x: 480, y: 959 },
    Point2d { x: 810, y: 959 },
    Point2d { x: 1135, y: 959 },
    Point2d { x: 1460, y: 959 },
    Point2d { x: 1790, y: 959 },
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
struct Point2d {
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

    fn from_colorchecker(image: ImageBuffer<Rgb<f32>, Vec<f32>>, coords: [Point2d; 24]) -> Self {
        let mut values = Vec::new();
        for i in coords.iter() {
            // values.push(image.get_pixel(i.x, i.y).0);
            values.push(Self::average_patch(image.clone(), i.x, i.y, 85));
        }
        Self::new(values)
    }

    // Very bad style
    fn average_patch(
        image: ImageBuffer<Rgb<f32>, Vec<f32>>,
        x: u32,
        y: u32,
        radius: u32,
    ) -> [f32; 3] {
        let (mut red, mut green, mut blue, mut num) = (0.0, 0.0, 0.0, 0.0);
        let (width, height) = image.dimensions();

        for i in (x - radius)..(x + radius) {
            for j in (y - radius)..(y + radius) {
                if i >= width || j >= height {
                    continue;
                }

                let c = image.get_pixel(i, j).0;
                red += c[0];
                green += c[1];
                blue += c[2];
                num += 1.0;
            }
        }

        [red / num, green / num, blue / num]
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
    name = "Get ColorChecker Values",
    about = USAGE_INSTRUCTIONS

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
