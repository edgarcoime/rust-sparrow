import "./index.css";
import { Viewport } from "./app/viewport";
import * as sim from "lib-simulation-wasm";
import { Terminal } from "./app/terminal";

const terminal = new Terminal(
  document.getElementById("terminal-stdin"),
  document.getElementById("terminal-stdout"),
);

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

terminal.onInput((input) => {
  terminal.println("");
  terminal.println("$ " + input);

  console.log(input)

  try {
    exec(input)
  } catch(err) {
    terminal.println(`  ^ err: ${err}`);
  }
});

function exec(input) {
  if (input.includes("[") || input.includes("]")) {
    throw "square brackets are for documentation purposes - you don't need to write them, eg.: reset animals=40"
  }

  const [cmd, ...args] = input.split(" ");

  if (cmd === "p" || cmd === "pause") {
    execPause(args)
    return;
  }

  throw "unknown command";
}

function execPause(args) {
  if (args.length > 0) {
    throw "this command accept no parameters";
  }

  active = !active;
}

function execReset(args) {
}


function execTrain(args) {
}

// region     Animation
const redraw = () => {
//   if (active) {
//     const stats = simulation.step();
//     console.log("Step")

//     if (stats) {
//       terminal.println(stats);
//     }
//   }

  // console.log(active)
  const stats = simulation.step();

  if (stats) {
    terminal.println(stats);
  }

  const config = simulation.config();
  const world = simulation.world();

  viewport.clear();

  world.foods.forEach(food => {
    viewport.drawCircle(
      food.x,
      food.y,
      (config.food_size / 2.0),
      "rgb(0, 255, 128",
    );
  });

  world.animals.forEach(animal => {
    viewport.drawTriangle(
      animal.x,
      animal.y,
      config.food_size,
      animal.rotation,
      "rgb(255, 255, 255)",
    );

    // const anglePerCell = config.eye_fov_angle / config.eye_cells;

    // for (let cellId = 0; cellId < config.eye_cells; cellId++) {
    //   const angleFrom = (animal.rotation - config.eye_fov_angle / 2.0) + (cellId * anglePerCell);
    //   const angleTo = angleFrom + anglePerCell;
    //   const energy = animal.vision[cellId];
  
    //   viewport.drawArc(
    //     animal.x,
    //     animal.y,
    //     (config.food_size * 2.5),
    //     angleFrom,
    //     angleTo,
    //     `rgba(0, 255, 120, ${energy})`
    //   );
    // }
  })

  requestAnimationFrame(redraw);
}
redraw()

// for (let i = 0; i < 25; i++) {
//   redraw();
// }










// const redraw = () => {
//   if (active) {
//     const stats = simulation.step();

//     if (stats) {
//       console.log(stats);
//       terminal.println(stats);
//     }
//   }

//   const config = simulation.config();
//   const world = simulation.world();

//   viewport.clear();

//   for (const food of world.foods) {
//     viewport.drawCircle(
//       food.x,
//       food.y,
//       config.food_size / 2.0,
//       "rgb(0, 255, 120)"
//     );
//   }

//   for (const animal of world.animals) {
//     viewport.drawTriangle(
//       animal.x,
//       animal.y,
//       config.food_size,
//       animal.rotation,
//       'rgb(255, 255, 255)',
//     );

//     const anglePerCell = config.eye_fov_angle / config.eye_cells;
  
//     for (let cellId = 0; cellId < config.eye_cells; cellId++) {
//       const angleFrom = (animal.rotation - config.eye_fov_angle / 2.0) + (cellId * anglePerCell);
//       const angleTo = angleFrom + anglePerCell;
//       const energy = animal.vision[cellId];
  
//       viewport.drawArc(
//         animal.x,
//         animal.y,
//         (config.food_size * 2.5),
//         angleFrom,
//         angleTo,
//         `rgba(0, 255, 120, ${energy})`
//       )
//     }
//   }

//   requestAnimationFrame(redraw);
// };

// // var fps = 60;
// // var now;
// // var then = Date.now()
// // var interval = 1000 / fps;
// // var delta;

// // function animationLoop() {
// //   requestAnimationFrame(animationLoop);

// //   now = Date.now()
// //   delta = now - then;

// //   if (delta > interval) {
// //     then = now - (delta % interval);

// //     redraw()
// //   }
// // }

// // animationLoop();
// redraw()
// // endregion: Animation
