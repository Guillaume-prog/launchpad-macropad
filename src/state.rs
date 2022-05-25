use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use configparser::ini::Ini;

// --------------------------------

pub struct GlobalState {
    preset_tab: u8,
    commands: Ini
}

impl GlobalState {

    pub fn new(data_path: &str) -> Self {
        return Self {
            preset_tab: 0,
            commands: GlobalState::load_commands(String::from(data_path))
        }
    }

    fn load_commands(path: String) -> Ini {
        let mut cmd_file = File::open(path).expect("Couldn't open data file");

        let mut contents = String::new();
        cmd_file.read_to_string(&mut contents).expect("Can't read file");

        let mut parser = Ini::new();
        parser.read(contents).expect("Couldn't read config");

        return parser;
    }

    pub fn set_preset(&mut self, index: u8) {
        if index > 0 && index <= (self.commands.sections().len() as u8) {
            self.preset_tab = index - 1;
        }
    }

    pub fn get_preset(&self) -> u8 {
        return self.preset_tab + 1;
    }

    pub fn get_command(&mut self, num_command: u8) -> Option<String> {
        return self.commands.get(
            self.get_section().as_str(),
            format!("{}", num_command).as_str()
        );
    }

    pub fn get_commands(&mut self) -> &HashMap<String, Option<String>> {
        let cmds = self.commands.get_map_ref().get(self.get_section().as_str()).unwrap();
        return cmds;
    }

    pub fn get_section(&self) -> String {
        let mut sections = self.commands.sections();
        sections.sort();
        let section = sections.get(self.preset_tab as usize).unwrap();

        return section.clone();
    }
}