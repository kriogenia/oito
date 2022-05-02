use input::map_key;
use oito_core::core::OitoCore;
use render::Renderer;
use rom_loader::{desktop::FilePathLoader, RomLoader};
use sdl2::{event::Event, keyboard::Scancode};
use structopt::StructOpt;
use std::error::Error;

mod args;
mod input;
mod render;

const TICKS_PER_FRAME: usize = 10;

fn main() -> Result<(), Box<dyn Error>> {
	let args = args::Args::from_args();

    let renderer = Renderer::new(args.scale, args.bg.into(), args.fg.into());

    let sdl = sdl2::init().expect("error during SDL2 initialization");
    let video = sdl.video().expect("error during setup of SDL2 video");
    let window = video
        .window(
            "Oito SDL2 Emulator",
            renderer.scaled_width(),
            renderer.scaled_height(),
        )
        .position_centered()
        .opengl()
        .build()
        .expect("error building SDL2 window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("error building SDL2 canvas");
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl
        .event_pump()
        .expect("error obtaining the event SDL2 event pump");

    let mut oito = OitoCore::new();
    let loader = FilePathLoader::new(&args.file);
    oito.load(loader.rom());

    'gameloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } | Event::KeyDown{scancode: Some(Scancode::Escape), ..} => {
                    break 'gameloop;
                }
				Event::KeyDown { scancode, .. } => {
					if let Some(key) = map_key(scancode) {
						oito.key_press(key);
					}
				}
				Event::KeyUp { scancode, .. } => {
					if let Some(key) = map_key(scancode) {
						oito.key_release(key);
					}
				}
                _ => {}
            }
        }

		for _ in 0..TICKS_PER_FRAME {
			oito.tick()?;
		}
		oito.frame_tick();
        renderer.draw_frame(&oito, &mut canvas);
    }

    Ok(())
}
