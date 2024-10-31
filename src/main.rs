use rdev::{listen, Event, EventType};
use std::process;

struct State {
    word: String,
    tabs: i32,
}

impl State {
    fn assign(&mut self) {
        self.word.push_str(": ");
    }

    fn end_assignment(&mut self) {
        println!("{}{}", "\t".repeat((self.tabs.max(0)) as usize), self.word);
        self.word.clear();
    }

    fn end_block(&mut self) {
        self.tabs -= 1;
    }

    fn start_block(&mut self) {
        self.word.push(':');
        println!("{}{}", "\t".repeat((self.tabs.max(0)) as usize), self.word);
        self.word.clear();
        self.tabs += 1;
    }

    fn start_List_b(&mut self) {
        self.word.push(':');
        println!("{}{}", "\t".repeat((self.tabs.max(0)) as usize), self.word);
        self.word.clear();
        self.word.push('-');
        self.tabs += 1;
    }

    fn end_List_b(&mut self) {
        println!("{}{}", "\t".repeat((self.tabs.max(0)) as usize), self.word);
        self.word.clear();
        self.tabs -= 1;
    }

    fn add_List_b_item(&mut self) {
        println!("{}{}", "\t".repeat((self.tabs.max(0)) as usize), self.word);
        self.word.clear();
        self.word.push('-');
    }

    fn new_character(&mut self, character: char) {
        self.word.push(character);
    }
}

fn main() {
    let mut state = State {
        word: String::new(),
        tabs: 0,
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
                '(' => state.start_List_b(),
                ')' => state.end_List_b(),
                '-' => state.add_List_b_item(),
                _ => state.new_character(character),
            }
        }
    }
}
