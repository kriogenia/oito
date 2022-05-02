import init, * as wasm from "./wasm.js";

const WIDTH = 64;
const HEIGHT = 48;
const SCALE = 15;
const TICKS_PER_FRAME = 10;

let current_frame = 0;

const canvas = document.getElementById("viewport");
canvas.width = WIDTH * SCALE;
canvas.height = HEIGHT * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

const input = document.getElementById("rom_input");

const run = async () => {
	await init();

	let oito = new wasm.OitoWasm();

	document.addEventListener("keydown", (e) => {
		console.log(e);
	});

	document.addEventListener("keyup", (e) => {
		console.log(e);
	});

	input.addEventListener(
		"change",
		(e) => {
			if (current_frame != 0) {
				window.cancelAnimationFrame(anim_frame);
			}

			let file = e.target.files[0];
			if (!file) {
				alert("Fail reading the ROM");
				return;
			}

			let fr = new FileReader();
			fr.onload = (_) => {
				let buffer = fr.result;
				const rom = new Uint8Array(buffer);
				oito.reset();
				oito.load(rom);
				mainloop(oito);
			};
			fr.readAsArrayBuffer(file);
		},
		false
	);
};

const mainloop = (oito) => {
	for (let i = 0; i < TICKS_PER_FRAME; i++) {
		oito.tick();
	}
	oito.tick_timers();

	ctx.fillStyle = "black";
	ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

	ctx.fillStyle = "white";
	oito.draw_screen(SCALE);

	current_frame = window.requestAnimationFrame(() => {
		mainloop(oito);
	});
};

run().catch(console.error);
