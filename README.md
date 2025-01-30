# Family Pro Game Asset Viewer

A Rust-based project to decode and view graphical assets from old DOS games developed by Family Pro, an obscure Korean game developer.

## Overview

This project aims to reverse-engineer and display the graphical assets from a selection of classic DOS games created by Family Pro. The goal is to provide a platform for enthusiasts and researchers to explore and learn more about these hidden gems of gaming history.

## Features

* **Asset Decoding**: A Rust library that decodes the graphical assets from Family Pro's DOS games, including tilesets (.fbk), sprites (.fx4), and backgrounds (.hmg).
* **Viewer App**: A simple command-line application that displays decoded assets.

## How to Use

1. Clone this repository: `git clone https://github.com/your-username/family-pro-game-asset-viewer.git`
2. Build the project using Cargo: `cargo build`
3. Run the viewer app with a specific asset file as an argument: `cargo run -- <asset-file>` (replace <asset-file> with the actual file name)
4. Enjoy viewing the decoded graphical assets!

## Contributing

This is an open-source project I did to help me learn Rust! If you're interested in helping with:

* Adding support for more asset formats
* Improving the viewer app's user interface
* Reverse-engineering new Family Pro games
* Please create a pull request or issue a bug report.

## License
This project is licensed under the MIT License. See LICENSE file for details.

## Acknowledgments
Special thanks to the Family Pro game development team and all those who have contributed to the preservation of these classic DOS games.

Happy gaming
