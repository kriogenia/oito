use input::map_key;
use oito_core::core::OitoCore;
use render::Renderer;
use rom_loader::{desktop::FilePathLoader, RomLoader};
use sdl2::{event::Event, keyboard::Scancode};
use sound::{SOUND_SPEC, Beep};
use structopt::StructOpt;
use std::error::Error;

mod args;
mod input;
mod render;
mod sound;

const TICKS_PER_FRAME: usize = 10;
const SOUND_PATH: &str = "./res/beep.wav";

fn main() -> Result<(), Box<dyn Error>> {
	let args = args::Args::from_args();

    let renderer = Renderer::new(args.scale, args.bg.into(), args.fg.into());

    let sdl = sdl2::init().expect("error during SDL2 initialization");
    let video = sdl.video().expect("error during video setup");
	let audio = sdl.audio().expect("error during audio setup");
	

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

	let beep = Beep::new(SOUND_PATH.to_string());
	let audio = audio.open_playback(None, &SOUND_SPEC, move |_| {
		beep
	}).expect("error during audio device setup");

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

		if oito.sound() {
			audio.resume();
		}

        renderer.draw_frame(&oito, &mut canvas);
    }

    Ok(())
}
