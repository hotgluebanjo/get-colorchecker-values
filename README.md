# Get ColorChecker Values

Extract datasets from ColorChecker patches.

## 🛠️ How to Build and Run It

You don't need to be a programmer to build this! Just follow the steps below.

### 📦 1. Install Rust

To build this project, you need to install Rust, a modern programming language 
that includes all the tools required for compiling this project.

#### Install Rust (via rustup)

Go to the official Rust installation page:
👉 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

##### On macOS or Linux:
Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then follow the on-screen instructions. Once installed, restart your terminal.

#####  On Windows:

Download and run the `.exe` installer from the Rust website. It’s a simple install, just like any other app.

🛠️ The installer might tell you to install [Visual Studio C++ Build Tools]((https://visualstudio.microsoft.com/downloads/?q=build+tools)).
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

### 🧱 2. Build the CLI

Navigate to the project folder using the terminal, then run:

```bash
cargo build --release
```

This compiles the project and creates the executable in the `target/release/`
directory.

📁 On Windows, the file will be called: `get-colorchecker-values.exe`

###  🚀 3. Run the CLI

Copy the previously generated `get-colorchecker-values.exe` to the folder where
you have the `.tiff` files. Then, open a terminal, navigate to that folder, and 
run the CLI.

#### Example usage:

🔍 Show Help Information:

```
get-colorchecker-values --help
```

📋 Print Dataset to the Terminal:

```
get-colorchecker-values chart.tiff
```

💾 Save Dataset to a `.txt` File:

```
get-colorchecker-values chart.tiff -o chart.txt
```