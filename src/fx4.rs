use olc_pixel_game_engine::Sprite;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::str::from_utf8;

use crate::animation::AnimatedSprite;
use crate::file::FileReader;
use crate::pal;

#[derive(Clone, Debug)]
struct FX4FileHeader {
    identifier: String,
    total_frames: i32,
}

#[derive(Clone, Debug)]
struct FX4FrameHeader {
    width: i32,
    height: i32,
    bytes: i16,
}

fn read_file_header(stream: &mut BufReader<File>) -> FX4FileHeader {
    let mut header: [u8; 30] = [0; 30];
    stream.read_exact(&mut header).unwrap();
    FX4FileHeader {
        identifier: String::from(from_utf8(&header[0..27]).expect("invalid utf-8 sequence")),
        total_frames: header[28] as u8 as i32 + 1,
    }
}

fn read_frame_header(stream: &mut BufReader<File>) -> Result<FX4FrameHeader, io::Error> {
    let mut header: [u8; 4] = [0; 4];
    match stream.read_exact(&mut header) {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    let (_, bytes_slice) = header.split_at(2);
    let bytes_array: [u8; 2] = bytes_slice.try_into().unwrap();

    Ok(FX4FrameHeader {
        width: header[0] as u8 as i32,
        height: header[1] as u8 as i32,
        bytes: i16::from_be_bytes(bytes_array),
    })
}

pub fn load_animation_from_fx4(sprite_file_name: String, pal_file_name: String) -> AnimatedSprite {
    let palette = pal::load_palette_from_file(pal_file_name);
    let file = FileReader::new(sprite_file_name);

    let mut istream = file.stream();
    let header = read_file_header(&mut istream);

    println!("File data:");
    println!("  Identifier: {:?}", header.identifier);
    println!("  Total frames: {:?}", header.total_frames);
    let mut a_sprite = AnimatedSprite::new();
    let mut total_frames = header.total_frames;
    let mut buffer = [0u8];
    while total_frames > 0 {
        let frame_header = read_frame_header(&mut istream).unwrap();
        println!(
            "  Frame {}: [width({}), height({}), bytes({})]",
            total_frames, frame_header.width, frame_header.height, frame_header.bytes
        );
        let mut frame = Sprite::with_dims(frame_header.width, frame_header.height);
        let mut x = 0;
        let mut y = 0;
        let frame_bytes = istream.stream_position().unwrap() + frame_header.bytes as u64;
        'frame: while istream.stream_position().unwrap() < frame_bytes {
            match istream.read_exact(&mut buffer) {
                Ok(buffer) => buffer,
                Err(error) => break 'frame,
            }
            if buffer[0] == 0xff {
                x = 0;
                y += 1;
                continue 'frame;
            }
            x = buffer[0] as u8 as i32 + x;
            match istream.read_exact(&mut buffer) {
                Ok(buffer) => buffer,
                Err(error) => break 'frame,
            }
            let segment_length: i32 = buffer[0] as u8 as i32;
            for i in 0..segment_length {
                match istream.read_exact(&mut buffer) {
                    Ok(buffer) => buffer,
                    Err(error) => break 'frame,
                }
                let color = palette[buffer[0] as usize];
                frame.set_pixel(x + i as i32, y as i32, color);
            }
            x += segment_length;
        }
        total_frames -= 1;
        a_sprite.push_frame(frame);
        println!(
            " frame end at position {}",
            istream.stream_position().unwrap()
        );
    }
    file.close();
    a_sprite
}
