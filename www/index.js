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

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size) {
  this.beginPath();
  this.moveTo(x, y);
  this.lineTo(x + size, y + size);
  this.lineTo(x - size, y + size);
  this.lineTo(x, y);

  this.fillStyle = 'rgb(0, 0, 0)';
  this.fill();
}

const main = () => {
  const simulation = sim.Simulation.new();

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
    )
  }
}

main()