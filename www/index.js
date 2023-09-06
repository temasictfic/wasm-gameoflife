import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 4; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const width = 192;
const height = 96;
let universe = Universe.new(width, height);


// Give the canvas room for all of our nexts and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;
let stepValue = 1;
let slider= document.getElementById("many-ticks");

slider.onchange = () => {
  stepValue = slider.value
}

const tick = () => {

  for (let index = 0; index < stepValue; index++) {

    universe.tick();

  }

};

const renderLoop = () => {
  //debugger; //pause each tick to inspect
  
  fps.render();
  
  tick();

  drawCells();

  animationId = requestAnimationFrame(renderLoop);

};

const isPaused = () => {
  return animationId === null;
};

const resetButton = document.getElementById("reset");

resetButton.addEventListener("click", event => {
  universe = Universe.new(width, height);
  drawCells();
  pause();
});

const clearButton = document.getElementById("clear");

clearButton.addEventListener("click", event => { 
  universe = Universe.new_empty(width, height);
  drawCells();
  pause();
});

const stepButton = document.getElementById("step");

stepButton.addEventListener("click", event => {   
  tick();
  drawCells();
  pause();
});

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);  // 8 bit = 1 byte
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const nextsPtr = universe.nexts();
  const nexts = new Uint8Array(memory.buffer, nextsPtr, width * height / 8);  // 8 bit = 1 byte

  ctx.beginPath();

  ctx.fillStyle = DEAD_COLOR;
  ctx.fillRect(0, 0, width * (CELL_SIZE + 1) + 1, height * (CELL_SIZE + 1) + 1);

  drawGrid();
  
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (bitIsSet(idx, nexts)) {

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  }

  ctx.stroke();

};

const getCord = (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
  return [row, col];
}

let dragging = false;

const setCell = (event) => {
  const [row, col] = getCord(event);
  universe.set_cell(row, col);
  drawCells();
}

canvas.addEventListener('mousedown', () => {
  dragging = true;
});

canvas.addEventListener('mousemove', event => {
  if (dragging === true) {
    setCell(event);
  }
});

window.addEventListener('mouseup', () => {
  if (dragging === true) {
    dragging = false;
  }
});

canvas.addEventListener("click", event => {
 
  const [row, col] = getCord(event);

  if (event.ctrlKey) {
    universe.glider(row, col);
  } else if (event.shiftKey) {
    universe.pulsar(row, col);
  } else {
    universe.toggle_cell(row, col);
  }

  drawCells();

});

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
latest          = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
    `.trim();
  }
};

// This used to be `requestAnimationFrame(renderLoop)`.
play();