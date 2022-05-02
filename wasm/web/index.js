import init, * as wasm from "./wasm.js";

const WIDTH = 64;
const HEIGHT = 48;
const SCALE = 15;
const TICKS_PER_FRAME = 10;

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
	console.log(oito);
};

run().catch(console.error);
