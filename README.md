# Get ColorChecker Values

Extract datasets from ColorChecker patches.

NOTE: This tool is very rudimentary. Prefer [`measure-colour-patches`](https://github.com/Amaraldo/measure-colour-patches).

## Usage

```
get-colorchecker-values -h
```

```
Get ColorChecker Values 0.2.5
Extract datasets from a ColorChecker.

Scale up and stretch the image until edges of the color patches hit the edges of the frame

USAGE:
    get-colorchecker-values.exe [FLAGS] [OPTIONS] <path>

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    Iterate through each file in the provided directory - Use . for the current one
    -V, --version      Prints version information

OPTIONS:
    -o, --output-name <output-name>    Name of file to write dataset to if desired

ARGS:
    <path>    Path to the image(s) of the ColorChecker
```

## Very Detailed Instructions

Scale your charts such that the outer border of the patches touch the edges of frame. Like [this](https://raw.githubusercontent.com/hotgluebanjo/get-colorchecker-values/master/tests/chart.png). Make sure the images are in one of the [supported formats](https://github.com/image-rs/image?tab=readme-ov-file#supported-image-formats)—preferably EXR or TIFF.

Check the [releases](https://github.com/hotgluebanjo/get-colorchecker-values/releases/latest) for a compatible precompiled binary. If your OS and architecture combination are not there, you'll need to [build the tool yourself](#how-to-build).

Open a terminal and [`cd`](https://en.wikipedia.org/wiki/Cd_(command)) to the directory where the executable is located and try running it by typing the program name and hitting enter. You'll need to use a dot slash on most shells to specify that the executable is in the current directory, e.g:
```
./get-colorchecker-values
```

You should get:
```
error: The following required arguments were not provided:
    <path>

USAGE:
    get-colorchecker-values.exe [FLAGS] [OPTIONS] <path>

For more information try --help
```

This means the program is running properly, just without any instructions. If you don't get that, you'll probably need to give it permissions or make it executable. A search engine is your friend. On Unix, try:

```
chmod +x ./get-colorchecker-values
```

Once it's up and running, check out the help info with the `--help` or `-h` flag.
```
get-colorchecker-values --help
```

It will print:
```
Get ColorChecker Values 0.2.5
Extract datasets from a ColorChecker.

Scale up and stretch the image until edges of the color patches hit the edges of the frame

USAGE:
    get-colorchecker-values.exe [FLAGS] [OPTIONS] <path>

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    Iterate through each file in the provided directory - Use . for the current one
    -V, --version      Prints version information

OPTIONS:
    -o, --output-name <output-name>    Name of file to write dataset to if desired

ARGS:
    <path>    Path to the image(s) of the ColorChecker
```

There's only really two options/flags to care about, and one argument. Here's how it works. If you pass an image path to the program, it will do its extraction thing and print the values to the screen. If your image is in the current working directory, you'll likely need to prepend a `./` to the filename.

```
get-colorchecker-values example.tiff
```

This will print the dataset as a space-delimited list of triplets. For example:
```
0.23200172185897827 0.17972472310066223 0.15904557704925537
0.3746511936187744 0.297799289226532 0.2738438844680786
0.23555000126361847 0.26013505458831787 0.2910904288291931
0.21426674723625183 0.2071380466222763 0.1588325947523117
0.2921278476715088 0.281299352645874 0.323480486869812
...
```

Now that's fine, but it's handy to dump large datasets directly into a file, so use the `--output-name` or `-o` option to create that file and specify its name:
```
get-colorchecker-values -o example.txt example.tiff
```

But that's only one input image. Captured datasets usually contain multiple exposures. So use the `--recursive` or `-r` flag and pass a *directory* of images to iterate through as the path argument.
```
get-colorchecker-values -r -o example_dataset.txt all_my_images
```

Note that it will alphabetically iterate through the images and append each file's samples to the dataset, so the naming very much matters. If the sorted order is different between hypothetical source and target image directories, corresponding data points will have mismatched indices and the datasets will be unusable with interpolation, etc.

## How to Build

You don't need to be a programmer to build this! Just follow the steps below.

### 1. Install Rust

To build this project, you need to install Rust, a modern programming language 
that includes all the tools required for compiling this project.

#### Install Rust (via rustup)

Go to the official Rust installation page: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

##### On macOS or Linux:
Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then follow the on-screen instructions. Once installed, restart your terminal.

##### On Windows:

Download and run the `.exe` installer from the Rust website. It’s a simple install, just like any other app.

The installer might tell you to install [Visual Studio C++ Build Tools]((https://visualstudio.microsoft.com/downloads/?q=build+tools)).
If that happens:

* Download the installer from the link above
* Run it and select the C++ build tools component
* You can check this helpful [StackOverflow post for guidance](https://stackoverflow.com/questions/40504552/how-to-install-visual-c-build-tools)

Once Rust is installed, open a terminal and run:

```bash
rustc --version
```

You should see something like:

```
rustc 1.74.0 (or a similar version)
```

### 2. Build the CLI

Navigate to the project folder using the terminal, then run:

```bash
cargo build --release
```

This compiles the project and creates the `get-colorchecker-values` executable in the `target/release/`
directory. It'll have a `.exe` extension on Windows.
