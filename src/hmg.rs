use olc_pixel_game_engine as olc;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::from_utf8;

use crate::animation::AnimatedSprite;
use crate::file::FileReader;
use crate::pal;

struct HMGFileHeader {
    identifier: String,
    total_frames: i32,
}

struct HMGFrameHeader {
    width: i16,
    height: i16,
}

fn read_file_header(stream: &mut BufReader<File>) -> HMGFileHeader {
    let mut header: [u8; 32] = [0; 32];
    stream.read_exact(&mut header).unwrap();
    HMGFileHeader {
        identifier: String::from(from_utf8(&header[0..11]).expect("invalid utf-8 sequence")),
        total_frames: header[17] as i32 + 1,
    }
}

fn read_frame_header(stream: &mut BufReader<File>) -> HMGFrameHeader {
    let mut header: [u8; 4] = [0; 4];
    stream.read_exact(&mut header).unwrap();
    HMGFrameHeader {
        width: i16::from_le_bytes(header[0..2].try_into().unwrap()),
        height: i16::from_le_bytes(header[2..4].try_into().unwrap()),
    }
}

pub fn load_animation_from_hmg(sprite_file_name: String, pal_file_name: String) -> AnimatedSprite {
    let palette = pal::load_palette_from_file(pal_file_name);
    let file = FileReader::new(sprite_file_name);

    let mut istream = file.stream();
    let header = read_file_header(&mut istream);

    println!("File data:");
    println!("  Identifier: {:?}", header.identifier);
    println!("  Total frames: {:?}", header.total_frames);
    let mut a_sprite = AnimatedSprite::new();
    let mut buffer = [0u8];
    let mut tilemap = olc::Sprite::with_dims(320, 200);
    'draw: for i in 0..header.total_frames {
        let header = read_frame_header(&mut istream);
        let span_i = i * header.width as i32;
        let mut tale_y = span_i / 320;
        let tile_x = span_i - tale_y * 320;
        tale_y *= header.height as i32;
        // let mut frame = olc::Sprite::with_dims(header.width as i32, header.height as i32);
        for y in 0..header.height {
            for x in 0..header.width {
                match istream.read(&mut buffer) {
                    Ok(0) => break 'draw, // EOF reached
                    Ok(_) => {
                        let color = palette[buffer[0] as usize];
                        tilemap.set_pixel(tile_x + x as i32, tale_y + y as i32, color);
                    } // Print the byte as a character
                    Err(e) => break 'draw, // Handle read errors
                }
            }
        }
    }
    a_sprite.push_frame(tilemap);
    file.close();
    a_sprite
}
