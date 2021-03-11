import {
	Hack,
	default as init
} from './wasm_hack.js';

async function run() {
	let wasm = await init('./wasm_hack_bg.wasm');

	// Create new cpu emulator
	const hack = Hack.new();

	// Init 2d canvas to render Hack screen
	var canvas = document.getElementById("hack-canvas");
	var new_canvas = canvas.cloneNode(true);
	canvas.parentNode.replaceChild(new_canvas, canvas);
	canvas = new_canvas;
	canvas.height = 256;
	canvas.width = 512;
	const ctx = canvas.getContext('2d');

	// Input init
	window.addEventListener('keydown', function(event) {
		hack.set_key(event.keyCode);
		if ([32, 37, 38, 39, 40].indexOf(event.keyCode) > -1) {
			event.preventDefault();
		}
	}, false);

	window.addEventListener('keyup', function(event) {
		hack.release_key(event.keyCode);
	});
	
	// Init mobile buttons
	var buttons = document.querySelectorAll('.button');

	buttons.forEach((btn) => {

	  btn.addEventListener('mousedown', (event) => {
		hack.set_key(btn.value);
	  });
	  
	  btn.addEventListener('mouseup', (event) => {
		 hack.release_key(btn.value); 
	  });
	  
	  btn.addEventListener('touchstart', (event) => {
		hack.set_key(btn.value);
	  });
	  
	  btn.addEventListener('touchend', (event) => {
		 hack.release_key(btn.value); 
	  });
	});
	
	// Reset functionality
	const resetButton = document.getElementById("reset-button");
	resetButton.textContent = "↺";
	
	resetButton.addEventListener('click', function(event) {
		hack.reset();
	});
	
	// Select ROM functionality
	const select_rom = document.querySelector('.select_rom');

	select_rom.addEventListener('change', (event) => {
		fetch('rom/' + event.target.value)
			.then(response => response.text())
			.then(data => {
				hack.load_rom(data);
				hack.reset();
			});
	});
	
	// Basic renderloop
	const renderLoop = () => {

		// Tick 'speed_input' times
		hack.tick(document.getElementById("speed_input").value);
		// Draw Hack screen
		draw();

		requestAnimationFrame(renderLoop);
	};

	const draw = () => {
		var canvasWidth = canvas.width;
		var canvasHeight = canvas.height;
		
		
		const imageData = ctx.getImageData(0, 0, canvasWidth, canvasHeight);
		var data = imageData.data;
		
		// draw Hack screen into its internal buffer with pixels
		hack.draw();

		// get pointer to buffer with pixels
		let frame_ptr = hack.frame();
		
		// Interpret buffer with pixels as JS array
		const pixels = new Uint8Array(wasm.memory.buffer, frame_ptr, 512 * 256 * 4);
		
		// Set this buffer as image in canvas
		imageData.data.set(pixels);
		
		// Draw canvas
		ctx.putImageData(imageData, 0, 0);
	}

	draw();
	renderLoop();
}

// Run whole script
run()