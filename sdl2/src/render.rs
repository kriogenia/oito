use oito_core::{core::OitoCore, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::{rect::{Rect}, render::Canvas, video::Window, pixels::Color};

const DEFAULT_SCALE: u32 = 15;

pub struct Renderer {
    scale: u32,
}

impl Renderer {
    pub fn scaled_width(&self) -> u32 {
        self.scale * SCREEN_WIDTH as u32
    }

    pub fn scaled_height(&self) -> u32 {
        self.scale * SCREEN_HEIGHT as u32
    }

    /// Draws the current Oito frame into the SDL2 Canvas
    pub fn draw_frame(&self, oito: &OitoCore, canvas: &mut Canvas<Window>) {
		canvas.set_draw_color(Color::BLACK);
        canvas.clear();

		canvas.set_draw_color(Color::WHITE);
        for (i, pixel) in oito.frame_buffer().iter().enumerate() {
            if *pixel {
                let x = (i % SCREEN_WIDTH) as i32 * self.scale as i32;
                let y = (i / SCREEN_WIDTH) as i32 * self.scale as i32;
				let rect = Rect::new(x, y, self.scale, self.scale);

                canvas
                    .fill_rect(rect)
                    .expect("error drawing point");
            }
        }

        canvas.present();
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            scale: DEFAULT_SCALE,
        }
    }
}
