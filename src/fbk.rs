use olc_pixel_game_engine::Sprite;
use std::io::Read;

use crate::animation::AnimatedSprite;
use crate::file::FileReader;
use crate::pal;

struct FBKFileHeader {
    identifier: [u8; 30],
}

// pub fn load_sprite_from_fbk(sprite_file_name: String, pal_file_name: String) -> Sprite {
//     let palette = pal::load_palette_from_file(pal_file_name);
//     let file = FileReader::new(sprite_file_name);
//     let mut sprite = Sprite::with_dims(320, 200);

//     let mut istream = file.stream();
//     let mut header = FBKFileHeader {
//         identifier: [0; 30],
//     };
//     istream.read_exact(&mut header.identifier).unwrap();
//     println!("File data:");
//     println!(
//         "  Identifier: {:?}",
//         std::str::from_utf8(&header.identifier[0..29]).expect("invalid utf-8 sequence")
//     );
//     let mut buffer = [0u8];
//     let mut x: i32 = 0;
//     let mut y: i32 = 0;
//     'draw: loop {
//         match istream.read(&mut buffer) {
//             Ok(0) => break 'draw, // EOF reached
//             Ok(_) => {
//                 if buffer[0] > 0xc0 {
//                     let dx: i32 = (buffer[0] - 0xc1) as i32;
//                     istream.read_exact(&mut buffer).unwrap();
//                     let color = palette[buffer[0] as usize];
//                     for i in 0..=dx {
//                         sprite.set_pixel(x + i, y, color);
//                     }
//                     x += dx as i32;
//                 } else {
//                     let color = palette[buffer[0] as usize];
//                     sprite.set_pixel(x, y, color);
//                 }
//                 x += 1;
//                 if x > 319 {
//                     x = 0;
//                     y += 1;
//                 }
//             } // Print the byte as a character
//             Err(e) => break 'draw, // Handle read errors
//         }
//     }
//     file.close();
//     sprite
// }

pub fn load_animation_from_fbk(sprite_file_name: String, pal_file_name: String) -> AnimatedSprite {
    let palette = pal::load_palette_from_file(pal_file_name);
    let file = FileReader::new(sprite_file_name);
    let mut sprite = Sprite::with_dims(320, 200);

    let mut istream = file.stream();
    let mut header = FBKFileHeader {
        identifier: [0; 30],
    };
    istream.read_exact(&mut header.identifier).unwrap();
    println!("File data:");
    println!(
        "  Identifier: {:?}",
        std::str::from_utf8(&header.identifier[0..29]).expect("invalid utf-8 sequence")
    );
    let mut buffer = [0u8];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    'draw: loop {
        match istream.read(&mut buffer) {
            Ok(0) => break 'draw, // EOF reached
            Ok(_) => {
                if buffer[0] > 0xc0 {
                    let dx: i32 = (buffer[0] - 0xc1) as i32;
                    istream.read_exact(&mut buffer).unwrap();
                    let color = palette[buffer[0] as usize];
                    for i in 0..=dx {
                        sprite.set_pixel(x + i, y, color);
                    }
                    x += dx as i32;
                } else {
                    let color = palette[buffer[0] as usize];
                    sprite.set_pixel(x, y, color);
                }
                x += 1;
                if x > 319 {
                    x = 0;
                    y += 1;
                }
            } // Print the byte as a character
            Err(e) => break 'draw, // Handle read errors
        }
    }
    file.close();

    let mut a_sprite = AnimatedSprite::new();
    a_sprite.push_frame(sprite);

    a_sprite
}
