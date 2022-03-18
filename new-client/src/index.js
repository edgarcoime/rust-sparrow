import "./index.css";
import { Viewport } from "./app/viewport";
import * as sim from "lib-simulation-wasm";

const viewport = new Viewport(document.getElementById("viewport"));

/**
 * Current simulation
 *
 * @type {Simulation}
 */
let simulation = new sim.Simulation(sim.Simulation.default_config());

/**
 * Determines whether the simulation is working or not.
 * Can be modified by pause command.
 *
 * @type {boolean}
 */
let active = true;

/**
 *  Current configuration of running simulation
 */
const config = simulation.config();

document.getElementById("train").onclick = function() {
  for (let i = 0; i < 50; i++) {
    console.log(simulation.train())
  }
}

const redraw = () => {
  if (active) {
    const stats = simulation.step();

    if (stats) {
      console.log(stats);
    }
  }

  const config = simulation.config();
  const world = simulation.world();

  viewport.clear();

  for (const food of world.foods) {
    viewport.drawCircle(
      food.x,
      food.y,
      config.food_size / 2.0,
      "rgb(0, 255, 120)"
    );
  }

  for (const animal of world.animals) {
    viewport.drawTriangle(
      animal.x,
      animal.y,
      config.food_size,
      animal.rotation,
      'rgb(255, 255, 255)',
    );

    const anglePerCell = config.eye_fov_angle / config.eye_cells;
  
    for (let cellId = 0; cellId < config.eye_cells; cellId++) {
      const angleFrom = (animal.rotation - config.eye_fov_angle / 2.0) + (cellId * anglePerCell);
      const angleTo = angleFrom + anglePerCell;
      const energy = animal.vision[cellId];
  
      viewport.drawArc(
        animal.x,
        animal.y,
        (config.food_size * 2.5),
        angleFrom,
        angleTo,
        `rgba(0, 255, 120, ${energy})`
      )
    }
  }
};


// region     Animation
var fps = 60;
var now;
var then = Date.now()
var interval = 1000 / fps;
var delta;

function animationLoop() {
  requestAnimationFrame(animationLoop);

  now = Date.now()
  delta = now - then;

  if (delta > interval) {
    then = now - (delta % interval);

    redraw()
  }
}

animationLoop();
// endregion: Animation
