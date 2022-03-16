import * as sim from "lib-simulation-wasm";

// const simulation = new sim.Simulation(); // <- BIG Difference you must REACH out to rust for current memory
//                                          // rather than creating an instance of simulation in js
const simulation = sim.Simulation.new();
console.log(simulation.world());
// const world = simulation.world();
// console.log(world);