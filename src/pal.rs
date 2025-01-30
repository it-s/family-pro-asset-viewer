use olc_pixel_game_engine::Pixel;
use std::io::Read;

use crate::file::FileReader;

pub fn load_palette_from_file(file_name: String) -> [Pixel; 256] {
    let file = FileReader::new(file_name);
    let mut colors: [Pixel; 256] = [Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }; 256];
    let mut istream = file.stream();
    for i in 0..256 {
        let mut rgb = [0; 3];
        istream.read(&mut rgb).unwrap();
        let color = Pixel::rgb(rgb[0] * 4, rgb[1] * 4, rgb[2] * 4);
        colors[i] = color;
    }
    file.close();

    colors
}
