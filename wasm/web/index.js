import init, * as wasm from "./wasm.js";

const WIDTH = 64;
const HEIGHT = 32;
const TICKS_PER_FRAME = 10;

let current_frame = 0;
let background = "#000000";
let foreground = "#ffffff";
let scale = 12;

const audio = new Audio("beep.wav");

const canvas = document.getElementById("viewport");
canvas.width = WIDTH * scale;
canvas.height = HEIGHT * scale;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * scale, HEIGHT * scale);

const input = document.getElementById("rom_input");
const bg_picker = document.getElementById("bg_picker");
const fg_picker = document.getElementById("fg_picker");
const scale_picker = document.getElementById("scale_picker");

const run = async () => {
	await init();

	let oito = new wasm.OitoWasm();

	document.addEventListener("keydown", (e) => {
		oito.key_press(e);
	});

	document.addEventListener("keyup", (e) => {
		oito.key_release(e);
	});

	bg_picker.addEventListener(
		"change",
		(e) => {
			background = e.target.value;
		},
		false
	);

	fg_picker.addEventListener(
		"change",
		(e) => {
			foreground = e.target.value;
		},
		false
	);

	scale_picker.addEventListener(
		"change",
		(e) => {
			scale = e.target.value;
			canvas.width = WIDTH * scale;
			canvas.height = HEIGHT * scale;
		},
		false
	)

	input.addEventListener(
		"change",
		(e) => {
			if (current_frame != 0) {
				window.cancelAnimationFrame(current_frame);
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
				gameloop(oito);
			};
			fr.readAsArrayBuffer(file);
		},
		false
	);
};

const gameloop = (oito) => {
	for (let i = 0; i < TICKS_PER_FRAME; i++) {
		oito.tick();
	}
	oito.frame_tick();

	ctx.fillStyle = background;
	ctx.fillRect(0, 0, WIDTH * scale, HEIGHT * scale);

	ctx.fillStyle = foreground;
	oito.draw(scale);

	current_frame = window.requestAnimationFrame(() => {
		gameloop(oito);
	});

	if (oito.sound()) {
		audio.play();
	}
};

run().catch(console.error);
