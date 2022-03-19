import * as sim from "lib-simulation-wasm";
import { Terminal } from "./app/terminal";
import { Viewport } from "./app/viewport";
import { introduction } from "./app/introduction";

/* ---------- */

const terminal = new Terminal(
  document.getElementById("terminal-stdin"),
  document.getElementById("terminal-stdout")
);

const viewport = new Viewport(document.getElementById("viewport"));

/**
 * Current simulation.
 *
 * @type {Simulation}
 */
let simulation = new sim.Simulation(sim.Simulation.default_config());

/**
 * Whether the simulation is working or not.
 * Can be modified by the `pause` command.
 *
 * @type {boolean}
 */
let active = true;

/* ---------- */

const config = simulation.config();
introduction(terminal, config);

/* ---------- */

terminal.onInput((input) => {
  terminal.println("");
  terminal.println("$ " + input);

  try {
    exec(input);
  } catch (err) {
    terminal.println(`  ^ err: ${err}`);
  }
});

function exec(input) {
  if (input.includes("[") || input.includes("]")) {
    throw "square brackets are just for documentation purposes - you don't have to write them, e.g.: reset animals=100";
  }

  const [cmd, ...args] = input.split(" ");

  if (cmd === "p" || cmd === "pause") {
    execPause(args);
    return;
  }

  if (cmd === "r" || cmd === "reset") {
    execReset(args);
    return;
  }

  if (cmd === "t" || cmd === "train") {
    execTrain(args);
    return;
  }

  throw "unknown command";
}

function execPause(args) {
  if (args.length > 0) {
    throw "this command accepts no parameters";
  }

  active = !active;
}

function execReset(args) {
  let config = sim.Simulation.default_config();

  for (const arg of args) {
    const [argName, argValue] = arg.split("=");

    if (argName.startsWith("i:")) {
      config[argName.slice(2)] = parseInt(argValue);
    } else if (argName.startsWith("f:")) {
      config[argName.slice(2)] = parseFloat(argValue);
    } else {
      switch (argName) {
        case "a":
        case "animals":
          config.world_animals = parseInt(argValue);
          break;

        case "f":
        case "foods":
          config.world_foods = parseInt(argValue);
          break;

        case "n":
        case "neurons":
          config.brain_neurons = parseInt(argValue);
          break;

        case "p":
        case "photoreceptors":
          config.eye_cells = parseInt(argValue);
          break;

        default:
          throw `unknown parameter: ${argName}`;
      }
    }
  }

  simulation = new sim.Simulation(config);
}

function execTrain(args) {
  if (args.length > 1) {
    throw "this command accepts at most one parameter";
  }

  const generations = args.length == 0 ? 1 : parseInt(args[0]);

  for (let i = 0; i < generations; i += 1) {
    if (i > 0) {
      terminal.println("");
    }

    const stats = simulation.train();
    terminal.println(stats);
  }
}

/* ---------- */
// region:  Animation

function redraw() {
  if (active) {
    const stats = simulation.step();

    if (stats) {
      terminal.println(stats);
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
      "rgb(0, 255, 128)"
    );
  }

  for (const animal of world.animals) {
    viewport.drawTriangle(
      animal.x,
      animal.y,
      config.food_size,
      animal.rotation,
      "rgb(255, 255, 255)"
    );

    const anglePerCell = config.eye_fov_angle / config.eye_cells;

    for (let cellId = 0; cellId < config.eye_cells; cellId += 1) {
      const angleFrom =
        animal.rotation - config.eye_fov_angle / 2.0 + cellId * anglePerCell;
      const angleTo = angleFrom + anglePerCell;
      const energy = animal.vision[cellId];

      viewport.drawArc(
        animal.x,
        animal.y,
        config.food_size * 2.5,
        angleFrom,
        angleTo,
        `rgba(0, 255, 128, ${energy})`
      );
    }
  }
}

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
// redraw()
// endregion: Animation