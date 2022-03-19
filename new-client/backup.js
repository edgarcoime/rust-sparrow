function exec(input) {
  let config = sim.simulation.default_config();

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
          throw `unkown parameter: ${argName}`
      }
    }
  }

  simulation = new sim.Simulation(config);jk
}