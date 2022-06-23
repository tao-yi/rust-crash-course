use crate::garden::vegetables::Asparagus;

mod control_flow;
mod enums_pattern_matching;
mod functions;
mod guessing_game;
mod ownership_rules;
mod ownership_simple;
mod slice_type;
mod struct_methods;
mod structs;
mod variables_and_mutability;

// tell the compiler to include the code it finds in src/garden.rs
pub mod garden;

fn main() {
    // ownership_simple::run();
    // guessing_game::run();
    // variables_and_mutability::run();
    // functions::run();
    // control_flow::run();
    // ownership_rules::run();
    // slice_type::run();
    // structs::run()
    // struct_methods::run();
    enums_pattern_matching::run();
    Asparagus {};
}
