import * as sim from "lib-simulation-wasm";

// const simulation = new sim.Simulation(); // <- BIG Difference you must REACH out to rust for current memory
//                                          // rather than creating an instance of simulation in js
const simulation = sim.Simulation.new();
console.log(simulation.world());
// const world = simulation.world();
// console.log(world);

const viewport = document.getElementById("viewport");
const ctx = viewport.getContext("2d");

// Determines color of the upcoming shape.
ctx.fillStyle = 'rgb(255, 0, 0)';

// Draws a rectangle filled with color determined by `fillStyle`
ctx.fillRect(10, 10, 100, 50) // X Y W H