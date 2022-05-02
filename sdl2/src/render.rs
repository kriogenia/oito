use oito_core::{core::OitoCore, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Renderer {
    scale: u32,
    bg: Color,
    fg: Color,
}

impl Renderer {
    pub fn new(scale: u32, bg: Color, fg: Color) -> Self {
        Self { scale, bg, fg }
    }

    pub fn scaled_width(&self) -> u32 {
        self.scale * SCREEN_WIDTH as u32
    }

    pub fn scaled_height(&self) -> u32 {
        self.scale * SCREEN_HEIGHT as u32
    }

    /// Draws the current Oito frame into the SDL2 Canvas
    pub fn draw_frame(&self, oito: &OitoCore, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.bg);
        canvas.clear();

        canvas.set_draw_color(self.fg);
        for (i, pixel) in oito.frame_buffer().iter().enumerate() {
            if *pixel {
                let x = (i % SCREEN_WIDTH) as i32 * self.scale as i32;
                let y = (i / SCREEN_WIDTH) as i32 * self.scale as i32;
                let rect = Rect::new(x, y, self.scale, self.scale);

                canvas.fill_rect(rect).expect("error drawing point");
            }
        }

        canvas.present();
    }
}