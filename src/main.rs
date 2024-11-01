pub mod file_writer;

use rdev::{listen, Event, EventType};
use std::{mem, process};

struct State {
    word: String,
    tabs: i32,
    filepath: Vec<u8>,
}

impl State {
    fn get_filepath(&mut self) -> String {
        String::from_utf8(unsafe { mem::transmute(self.filepath.clone()) }).unwrap()
    }

    fn get_formatted_line(&mut self) -> String {
        return format!("{}{}", "\t".repeat((self.tabs.max(0)) as usize), self.word);
    }

    fn assign(&mut self) {
        self.word.push_str(": ");
    }

    fn end_assignment(&mut self) {
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
    }

    fn end_block(&mut self) {
        self.tabs -= 1;
    }

    fn start_block(&mut self) {
        self.word.push(':');
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.tabs += 1;
    }

    fn start_list_b(&mut self) {
        self.word.push(':');
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.word.push('-');
        self.tabs += 1;
    }

    fn end_list_b(&mut self) {
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.tabs -= 1;
    }

    fn add_list_b_item(&mut self) {
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.word.push('-');
    }

    fn new_character(&mut self, character: char) {
        self.word.push(character);
    }
}

fn main() {
    let _ = file_writer::make_file("test.yml");
    let mut state = State {
        word: String::new(),
        tabs: 0,
        filepath: String::from("test.yml").into_bytes(),
    };

    if let Err(error) = listen(move |event| callback(event, &mut state)) {
        println!("Error: {:?}", error);
        process::exit(1);
    }
}

fn callback(event: Event, state: &mut State) {
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
                _ => state.new_character(character),
            }
        }
    }
}
