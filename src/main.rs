use control_flow::run;

mod control_flow;
mod functions;
mod guessing_game;
mod ownership_simple;
mod variables_and_mutability;

fn main() {
    // ownership_simple::run();
    // guessing_game::run();
    variables_and_mutability::run();
    functions::run();
    control_flow::run();
}
