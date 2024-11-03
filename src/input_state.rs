use std::mem;

use crate::{deployment::deployment_container, file_writer};

pub(crate) struct State {
    pub(crate) word: String,
    pub(crate) tabs: i32,
    pub(crate) filepath: Vec<u8>,
}

impl State {
    pub(crate) fn get_filepath(&mut self) -> String {
        String::from_utf8(unsafe { mem::transmute(self.filepath.clone()) }).unwrap()
    }

    pub(crate) fn get_formatted_line(&mut self) -> String {
        return format!("{}{}", "\t".repeat((self.tabs.max(0)) as usize), self.word);
    }

    pub(crate) fn generate_deployment(&mut self, deployment: &mut deployment_container) {
        let _ = file_writer::write(&self.get_filepath(), &deployment.get_deployment());
    }

    pub(crate) fn assign(&mut self) {
        self.word.push_str(": ");
    }

    pub(crate) fn end_assignment(&mut self) {
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
    }

    pub(crate) fn end_block(&mut self) {
        self.tabs -= 1;
    }

    pub(crate) fn start_block(&mut self) {
        self.word.push(':');
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.tabs += 1;
    }

    pub(crate) fn start_list_b(&mut self) {
        self.word.push(':');
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.word.push('-');
        self.tabs += 1;
    }

    pub(crate) fn end_list_b(&mut self) {
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.tabs -= 1;
    }

    pub(crate) fn add_list_b_item(&mut self) {
        let result: String = self.get_formatted_line();
        let _ = file_writer::write(&self.get_filepath(), &result);
        self.word.clear();
        self.word.push('-');
    }

    pub(crate) fn new_character(&mut self, character: char) {
        self.word.push(character);
    }
}
