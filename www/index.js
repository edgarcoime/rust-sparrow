import * as sim from "lib-simulation-wasm";

// const simulation = new sim.Simulation(); // <- BIG Difference you must REACH out to rust for current memory
//                                          // rather than creating an instance of simulation in js
// const simulation = sim.Simulation.new();
// console.log(simulation.world());
// const world = simulation.world();
// console.log(world);

// const viewport = document.getElementById("viewport");
// const ctx = viewport.getContext("2d");

// Determines color of the upcoming shape.
// ctx.fillStyle = 'rgb(255, 0, 0)';

// Draws a rectangle filled with color determined by `fillStyle`
// ctx.fillRect(10, 10, 100, 50) // X Y W H


CanvasRenderingContext2D.prototype.drawTriangle = 
  function (x, y, size, rotation) { 
    this.beginPath();

    this.moveTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5,
    );

    this.lineTo(
        x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
        x + Math.cos(rotation - 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation - 2.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5,
    );

    this.fillStyle = 'rgb(255, 255, 255)';
    this.fill();
};

CanvasRenderingContext2D.prototype.drawCircle = 
  function(x, y, radius) {
    this.beginPath();

    this.arc(x, y, radius, 0, 2.0 * Math.PI);

    this.fillStyle = 'rgb(0, 255, 128)';
    this.fill();
};

const simulation = sim.Simulation.new();
console.log(simulation.world().animals)

const viewport = document.getElementById("viewport");
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;
console.log(viewportScale)

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
viewport.style.width = viewportWidth + "px";
viewport.style.height = viewportHeight + "px";

const ctx = viewport.getContext("2d");
ctx.scale(viewportScale, viewportScale);
ctx.fillStyle = 'rgb(0,0,0)';

for (const animal of simulation.world().animals) {
  ctx.drawTriangle(
    animal.x * viewportWidth,
    animal.y * viewportHeight,
    0.01 * viewportWidth,
    animal.rotation,
  )
}

function redraw() {
  ctx.clearRect(0, 0, viewportWidth, viewportHeight);

  simulation.step();

  const world = simulation.world();

  for (const food of world.foods) {
    ctx.drawCircle(
      food.x * viewportWidth,
      food.y * viewportHeight,
      (0.01 / 2.0) * viewportWidth,
    );
  }

  for (const animal of world.animals) {
    ctx.drawTriangle(
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      0.01 * viewportWidth,
      animal.rotation,
    );
  }

  requestAnimationFrame(redraw);
}

document.getElementById("train").onclick = function () {
  simulation.train();
}

redraw();