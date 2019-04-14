use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use super::commands::Command;
use super::keyboard::KeyStroke;

#[derive(Debug, Default)]
pub struct Config {
    pub leader: Option<KeyStroke>,
    pub normal_mode: HashMap<KeyStroke, Command>,
    pub insert_mode: HashMap<KeyStroke, Command>,
    pub visual_mode: HashMap<KeyStroke, Command>,
    pub action_mode: HashMap<KeyStroke, Command>,
}

impl Config {
    pub fn from_config_dir(config_dir: &Path) -> Result<Self, io::Error> {
        let mut file = File::open(config_dir.join("keyboard.vim"))?;
        let mut raw_contents = String::new();
        file.read_to_string(&mut raw_contents)?;

        Self::from_str(&raw_contents)
    }

    fn parse_map(&mut self, line: &str) {
        let elements: Vec<&str> = line.split(' ').collect();

        if elements.len() < 3 {
            return;
        }

        let key = match KeyStroke::from_description(elements[1]) {
            Some(key) => key,
            None => return,
        };

        let command = match Command::from_description(elements[2]) {
            Some(cmd) => cmd,
            None => return,
        };

        match elements[0] {
            "imap" => self.insert_mode.insert(key, command),
            "nmap" => self.normal_mode.insert(key, command),
            "vmap" => self.visual_mode.insert(key, command),
            "map" => {
                self.normal_mode.insert(key, command);
                self.visual_mode.insert(key, command)
            }
            _ => return,
        };
    }

    fn parse_set(&mut self, line: &str) {
        let elements: Vec<&str> = line.split(' ').collect();

        if elements.len() != 3 {
            return;
        }

        match elements[1] {
            "leader" => self.leader = KeyStroke::from_description(elements[2]),
            _ => return,
        }
    }
}

impl FromStr for Config {
    type Err = io::Error;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut config = Self::default();

        for line in raw.lines() {
            match line.splitn(1, ' ').nth(0).unwrap_or_default() {
                "map" | "nmap" | "vmap" | "imap" => config.parse_map(line),
                "set" => config.parse_set(line),
                _ => (),
            }
        }

        Ok(config)
    }
}
