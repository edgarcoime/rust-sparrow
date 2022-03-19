export const introduction = (terminal, config) => {
  terminal.println("");
  terminal.println("  _________                                         ");
  terminal.println(" /   _____/__________ ______________  ______  _  __ ");
  terminal.println(" \\_____  \\\\____ \\__  \\\\_  __ \\_  __ \\/  _ \\ \\/ \\/ / ");
  terminal.println(" /        \\  |_> > __ \\|  | \\/|  | \\(  <_> )     /  ");
  terminal.println("/_______  /   __(____  /__|   |__|   \\____/ \\/\\_/   ");
  terminal.println("        \\/|__|       \\/                             ");
  terminal.println("");
  terminal.println(
    "Simulating evolution through an ever-growing neural network, powered by a genetic algorithm"
  );
  terminal.println("");
  terminal.println("---- About ----");
  terminal.println("");
  terminal.println(
    "Each triangle represents a bird; each bird has an *eye* that is able to see around the bird and a *brain* that decides where and how fast the bird should be moving."
  );
  terminal.println("");
  terminal.println(
    "Each circle represents a piece of food that birds are supposed to find and eat."
  );
  terminal.println("");
  terminal.println(
    "All birds start with randomized brains and after 2500 turns birds who manage to eat the most food are reproduced, and their offspring starts the simulation over again."
  );
  terminal.println("");
  terminal.println(
    "Thanks to evolution, every generation gets slightly better at locating the food - almost as if the birds programmed themselves!"
  );
  terminal.println("");
  terminal.println(
    "You can affect the simulation by entering commands in the input at the bottom of this box - for starters, try executing the `train` command a few times (write `t`, press enter, write `t`, press enter etc.) - this fast-forwards the simulation, allowing you to see the birds getting smarter by the second."
  );
  terminal.println("");
  terminal.println("Would you like to read the source code?");
  terminal.println("https://github.com/Patryk27/shorelark");
  terminal.println("");
  terminal.println("Have fun!");
  terminal.println("");
  terminal.println("---- Commands ----");
  terminal.println("");
  terminal.println("- p / pause");
  terminal.println("  Pauses (or resumes) the simulation");
  terminal.println("");
  terminal.println(
    `- r / reset [animals=${config.world_animals}] [f=${config.world_foods}] [...]`
  );
  terminal.println("  Starts simulation from scratch with given optional");
  terminal.println("  parameters:");
  terminal.println("");
  terminal.println(`  * a / animals (default=${config.world_animals})`);
  terminal.println("    number of animals");
  terminal.println("");
  terminal.println(`  * f / foods (default=${config.world_foods})`);
  terminal.println("    number of foods");
  terminal.println("");
  terminal.println(`  * n / neurons (default=${config.brain_neurons})`);
  terminal.println("    number of brain neurons per each animal");
  terminal.println("");
  terminal.println(`  * p / photoreceptors (default=${config.eye_cells})`);
  terminal.println("    number of eye cells per each animal");
  terminal.println("");
  terminal.println("  Examples:");
  terminal.println("    reset animals=100 foods=100");
  terminal.println("    r a=100 f=100");
  terminal.println("    r p=3");
  terminal.println("");
  terminal.println("- (t)rain [how-many-generations]");
  terminal.println("  Fast-forwards one or many generations, allowing to");
  terminal.println("  observe the learning process faster.");
  terminal.println("");
  terminal.println("  Examples:");
  terminal.println("    train");
  terminal.println("    t 5");
  terminal.println("");
  terminal.println("---- Advanced Tips™ ----");
  terminal.println("");
  terminal.println("- `reset` can modify *all* of the parameters:");
  terminal.println("");
  terminal.println("  * r i:integer_param=123 f:float_param=123");
  terminal.println("  * r a=200 f=200 f:food_size=0.002");
  terminal.println("");
  terminal.println("---- Funky scenarios ----");
  terminal.println("");
  terminal.println("  * r i:ga_reverse=1 f:sim_speed_min=0.003");
  terminal.println("    (birdies *avoid* food)");
  terminal.println("");
  terminal.println("  * r i:brain_neurons=1");
  terminal.println("    (single-neuroned zombies)");
  terminal.println("");
  terminal.println("  * r f:food_size=0.05");
  terminal.println("    (biiiigie birdies)");
  terminal.println("");
  terminal.println("  * r f:eye_fov_angle=0.45");
  terminal.println("    (narrow field of view)");
  terminal.println("");
  terminal.println("----");
  terminal.scrollToTop();
}