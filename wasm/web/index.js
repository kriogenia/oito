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

const run = async () => {
	await init();

	document.addEventListener("keydown", function (evt) {
		console.log(evt);
	});
	
	document.addEventListener("keyup", function (evt) {
		console.log(evt);
	});

};

run().catch(console.error);