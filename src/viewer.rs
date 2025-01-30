use olc_pixel_game_engine as olc;

use crate::{animation::AnimatedSprite, fbk, file, fx4, hmg};
use num::clamp;

const TICK: f32 = 10.0;
const TOCK: f32 = 4.0;

pub struct Viewer {
    tick: f32,
    num_ticks: i32,
    sprite: AnimatedSprite,
    sprite_x: i32,
    sprite_y: i32,
    sprite_scale: i32,
    mouse_offset: olc::Vi2d,
}

impl Viewer {
    pub fn new(sprite_file_name: String, pal_file_name: String, scale: i32) -> Self {
        let filetype = file::get_file_type(sprite_file_name.clone());
        println!("Sprite type: {:?}", filetype);
        let sprite: AnimatedSprite;

        match filetype {
            file::FileType::FBK => {
                sprite = fbk::load_animation_from_fbk(sprite_file_name, pal_file_name);
            }
            file::FileType::HMG => {
                sprite = hmg::load_animation_from_hmg(sprite_file_name, pal_file_name);
            }
            file::FileType::FX4 => {
                sprite = fx4::load_animation_from_fx4(sprite_file_name, pal_file_name);
            }
            _ => {
                panic!("Unknown file type")
            }
        }

        Viewer {
            tick: 0.0,
            num_ticks: 0,
            sprite,
            sprite_x: 0,
            sprite_y: 0,
            sprite_scale: scale,
            mouse_offset: olc::Vi2d::new(0, 0),
        }
    }

    fn normalize_scale(&mut self) {
        self.sprite_scale = clamp(self.sprite_scale, 1, 10);
        println!("Scale: {}", self.sprite_scale);
    }
}

impl olc::Application for Viewer {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }

    fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
        self.tick += TICK * elapsed_time;
        let is_tick = self.tick >= TOCK;
        if is_tick {
            self.tick -= TOCK;
            self.num_ticks += 1;
            self.sprite.next();
        }

        if olc::get_key(olc::Key::RIGHT).held {
            self.sprite_x += 1;
        }
        if olc::get_key(olc::Key::LEFT).held {
            self.sprite_x -= 1;
        }
        if olc::get_key(olc::Key::UP).held {
            self.sprite_y -= 1;
        }
        if olc::get_key(olc::Key::DOWN).held {
            self.sprite_y += 1;
        }
        if olc::get_key(olc::Key::NP_ADD).pressed {
            self.sprite_scale += 1;
            self.normalize_scale();
        }
        if olc::get_key(olc::Key::NP_SUB).pressed {
            self.sprite_scale -= 1;
            self.normalize_scale();
        }
        if olc::get_mouse(0).pressed {
            self.mouse_offset = olc::Vi2d::new(
                olc::get_mouse_x() - self.sprite_x,
                olc::get_mouse_y() - self.sprite_y,
            );
        }
        if olc::get_mouse(2).pressed {
            self.sprite_x = 0;
            self.sprite_y = 0;
        }
        if olc::get_mouse(0).held {
            self.sprite_x = olc::get_mouse_x() - self.mouse_offset.x;
            self.sprite_y = olc::get_mouse_y() - self.mouse_offset.y;
        }
        // Clears screen and sets black colour.
        olc::clear(olc::BLUE);

        let animated_sprite = &mut self.sprite.frame();
        olc::draw_partial_sprite_ext(
            self.sprite_x,
            self.sprite_y,
            animated_sprite,
            0,
            0,
            animated_sprite.width(),
            animated_sprite.height(),
            self.sprite_scale as u32,
            olc::SpriteFlip::NONE,
        );
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
        Ok(())
    }
}
