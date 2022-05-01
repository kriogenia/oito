use std::env;
use oito_core::*;
use sdl2::event::Event;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return;
    }

    let sdl = sdl2::init().expect("error during SDL2 initialization");
    let video = sdl.video().expect("error during setup of SDL2 video");
    let window = video
        .window("Oito SDL2 Emulator", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .expect("error building SDL2 window");
		
    let mut canvas = window.into_canvas().present_vsync().build().expect("error building SDL2 canvas");
    canvas.clear();
    canvas.present();

	let mut event_pump = sdl.event_pump().expect("error obtaining the event SDL2 event pump");

	'gameloop: loop {
		 for e in event_pump.poll_iter() {
			match e {
				Event::Quit{..} => {
					break 'gameloop;
				},
				_ => {}
			}
		 }
	}

}
