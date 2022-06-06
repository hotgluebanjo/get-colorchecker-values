// NOTE: The Y axis is inverted in the image crate. Or at least
// it's the opposite of Nuke. In Nuke, the bottom of the image
// is 0 and the top is 1079. Inverse of that here.
// Took forever to figure out.
use image::{ImageBuffer, ImageResult, Rgb};
use std::{fmt, fs, io::Write, path::PathBuf, process};
use structopt::StructOpt;

type Image = ImageBuffer<Rgb<f32>, Vec<f32>>;
type Triplet = [f32; 3];
type CheckerPoints<'a> = &'a [Point2d<f32>];

const USAGE_INSTRUCTIONS: &str = "Extract datasets from a ColorChecker.

Scale up and stretch the image until edges of the color patches hit the edges of the frame";

macro_rules! attempt {
    ($x:expr, $message:expr) => {
        $x.unwrap_or_else(|_| {
            eprintln!("Error: {}", $message);
            process::exit(1);
        })
    };
    ($x:expr) => {
        $x.unwrap_or_else(|e| {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        })
    };
}

// Relative to 1.0 instead of 1080 or a specific resolution.
const COLORCHECKER_CLASSIC: CheckerPoints = &[
    Point2d {
        x: 0.0755,
        y: 0.1194,
    },
    Point2d {
        x: 0.2500,
        y: 0.1194,
    },
    Point2d {
        x: 0.4218,
        y: 0.1194,
    },
    Point2d {
        x: 0.5911,
        y: 0.1194,
    },
    Point2d {
        x: 0.7604,
        y: 0.1194,
    },
    Point2d {
        x: 0.9322,
        y: 0.1194,
    },
    Point2d {
        x: 0.0755,
        y: 0.3750,
    },
    Point2d {
        x: 0.2500,
        y: 0.3750,
    },
    Point2d {
        x: 0.4218,
        y: 0.3750,
    },
    Point2d {
        x: 0.5911,
        y: 0.3750,
    },
    Point2d {
        x: 0.7604,
        y: 0.3750,
    },
    Point2d {
        x: 0.9322,
        y: 0.3750,
    },
    Point2d {
        x: 0.0755,
        y: 0.6287,
    },
    Point2d {
        x: 0.2500,
        y: 0.6287,
    },
    Point2d {
        x: 0.4218,
        y: 0.6287,
    },
    Point2d {
        x: 0.5911,
        y: 0.6287,
    },
    Point2d {
        x: 0.7604,
        y: 0.6287,
    },
    Point2d {
        x: 0.9322,
        y: 0.6287,
    },
    Point2d {
        x: 0.0755,
        y: 0.8879,
    },
    Point2d {
        x: 0.2500,
        y: 0.8879,
    },
    Point2d {
        x: 0.4218,
        y: 0.8879,
    },
    Point2d {
        x: 0.5911,
        y: 0.8879,
    },
    Point2d {
        x: 0.7604,
        y: 0.8879,
    },
    Point2d {
        x: 0.9322,
        y: 0.8879,
    },
];

/// Supported ColorCheckers
enum Colorchecker<'a> {
    /// Can be used for other similar ColorCheckers with a close enough pattern
    Classic(CheckerPoints<'a>),
}

impl Colorchecker<'_> {
    fn average_patch(image: Image, x: u32, y: u32, radius: u32) -> Triplet {
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

#[derive(Debug)]
struct Point2d<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Dataset(Vec<Triplet>);

impl Dataset {
    fn from_colorchecker(image: Image, colorchecker: Colorchecker) -> Self {
        let patch_positions = match colorchecker {
            Colorchecker::Classic(p) => p,
        };

        let (width, height) = image.dimensions();

        let values = patch_positions
            .iter()
            .map(|p| {
                let calc_width = (width as f32 * p.x) as u32;
                let calc_height = (height as f32 * p.y) as u32;

                Colorchecker::average_patch(image.clone(), calc_width, calc_height, 80)
            })
            .collect::<Vec<Triplet>>();

        Self(values)
    }
}

impl fmt::Display for Dataset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.0 {
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
    /// Path to the image(s) of the ColorChecker
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Name of file to write dataset to if desired
    #[structopt(short, long)]
    output_name: Option<String>,

    /// Iterate through each file in the provided directory - Use . for the current one
    #[structopt(short, long)]
    recursive: bool,
}

fn get_values(path: PathBuf, output_name: &Option<String>) -> ImageResult<()> {
    println!("Reading image..."); // Takes long enough
    let image = image::open(path)?.to_rgb32f();
    let colorchecker = Colorchecker::Classic(COLORCHECKER_CLASSIC);

    let dataset = Dataset::from_colorchecker(image, colorchecker);

    match output_name {
        Some(name) => {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(name)
                .unwrap();
            write!(file, "{}", dataset).unwrap();
        }
        None => print!("{}", dataset),
    }

    Ok(())
}

fn main() {
    let args = Cli::from_args();

    if args.recursive {
        let dir = attempt!(fs::read_dir(args.path), "Invalid directory");

        for i in dir {
            let file = i.unwrap().path();

            get_values(file, &args.output_name)
                .unwrap_or_else(|_| eprintln!("Failed to read file"));
        }
    } else {
        attempt!(get_values(args.path, &args.output_name), "Can't open image");
    }
}
