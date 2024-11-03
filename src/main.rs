mod deployment;
pub mod file_writer;
mod input_state;

use deployment::deployment_container;
use input_state::State;
use rdev::{listen, Event, EventType};
use std::io;
use std::{mem, process};

fn main() {
    let mut deployment = deployment_container::new();

    let _ = file_writer::make_file("test.yml");
    let mut state = State {
        word: String::new(),
        tabs: 0,
        filepath: String::from("test.yml").into_bytes(),
    };

    if let Err(error) = listen(move |event| callback(event, &mut state, &mut deployment)) {
        println!("Error: {:?}", error);
        process::exit(1);
    }
}

fn callback(event: Event, state: &mut State, deployment: &mut deployment_container) {
    if let EventType::KeyPress(_) = event.event_type {
        if let Some(character) = event.name.and_then(|f| f.parse().ok()) {
            match character {
                ':' => state.assign(),
                '.' => state.end_assignment(),
                '>' => state.end_block(),
                '<' => state.start_block(),
                '(' => state.start_list_b(),
                ')' => state.end_list_b(),
                '-' => state.add_list_b_item(),
                '!' => state.generate_deployment(deployment),
                _ => state.new_character(character),
            }
        }
    }
}
