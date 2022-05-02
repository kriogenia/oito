use oito_core::core::OitoCore;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OitoWasm {
	oito: OitoCore
}

#[wasm_bindgen]
impl OitoWasm {
	
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			oito: OitoCore::new()
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
	pub fn reset(&mut self) {
		self.oito = OitoCore::new();
	}

}