use olc_pixel_game_engine as olc;

pub struct AnimatedSprite {
    frames: Vec<olc::Sprite>,
    frame_index: i32,
    len: i32,
}

impl AnimatedSprite {
    /// Creates a new [`AnimatedSprite`].
    pub fn new() -> Self {
        AnimatedSprite {
            frames: Vec::new(),
            frame_index: 0,
            len: 0,
        }
    }
    pub fn push_frame(&mut self, frame: olc::Sprite) {
        self.frames.push(frame);
        self.len += 1;
    }
    pub fn next(&mut self) {
        self.frame_index += 1;
        if self.len == self.frame_index {
            self.frame_index = 0;
        }
    }
    pub fn frame(&mut self) -> &olc::Sprite {
        let frame = &self.frames[self.frame_index as usize];
        frame
    }
}
