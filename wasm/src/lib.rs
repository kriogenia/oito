use oito_core::{core::OitoCore, SCREEN_WIDTH, SCREEN_HEIGHT};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

mod input;

#[wasm_bindgen]
pub struct OitoWasm {
    oito: OitoCore,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl OitoWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("viewport").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self {
            oito: OitoCore::new(),
            ctx,
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.oito.tick().unwrap();
    }

    #[wasm_bindgen]
    pub fn frame_tick(&mut self) {
        self.oito.frame_tick();
    }

    #[wasm_bindgen]
    pub fn load(&mut self, data: js_sys::Uint8Array) {
        self.oito.load(&data.to_vec());
    }

    /// Emmulates the pressing of the desired key
    pub fn key_press(&mut self, key: KeyboardEvent) {
		let code = key.code();
		if let Some(key) = input::map_key(&code) {
			self.oito.key_press(key);
		}
    }

    /// Emmulates the release of the desired key
    pub fn key_release(&mut self, key: KeyboardEvent) {
        let code = key.code();
		if let Some(key) = input::map_key(&code) {
			self.oito.key_release(key);
		}
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, scale: usize) {
        let disp = self.oito.frame_buffer();
        for i in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
            if disp[i] {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }

	#[wasm_bindgen]
	pub fn sound(&self) -> bool {
		self.oito.sound()
	}

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.oito = OitoCore::new();
    }
}
