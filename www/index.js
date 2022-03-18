import * as sim from "lib-simulation-wasm";
import { Viewport } from "./app/viewport";

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


  requestAnimationFrame(redraw);
};

redraw()
