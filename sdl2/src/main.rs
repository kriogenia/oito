use oito_core::core::OitoCore;
use render::Renderer;
use rom_loader::{desktop::FilePathLoader, RomLoader};
use sdl2::event::Event;
use std::{env, error::Error};

mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from("Usage: cargo run path/to/rom").into());
    }

    let path = if args[1] == "test" {
        "../rom_loader/test/test_opcode.ch8"
    } else {
        &args[1]
    };

    let renderer = Renderer::default();

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
    let loader = FilePathLoader::new(path);
    oito.load(loader.rom());

    'gameloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => {}
            }
        }

        oito.tick()?;
        renderer.draw_frame(&oito, &mut canvas);
    }

    Ok(())
}
