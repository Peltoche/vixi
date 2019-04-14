use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use super::commands::Command;
use super::keyboard::KeyStroke;

#[derive(Debug, Default)]
pub struct Config {
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
}

impl FromStr for Config {
    type Err = io::Error;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut config = Self::default();

        for line in raw.lines() {
            let elements: Vec<&str> = line.split(' ').collect();

            if elements.len() < 3 {
                continue;
            }

            let key = match KeyStroke::from_description(elements[1]) {
                Some(key) => key,
                None => continue,
            };

            let command = match Command::from_description(elements[2]) {
                Some(cmd) => cmd,
                None => continue,
            };

            match elements[0] {
                "imap" => config.insert_mode.insert(key, command),
                "nmap" => config.normal_mode.insert(key, command),
                "vmap" => config.visual_mode.insert(key, command),
                "map" => {
                    config.normal_mode.insert(key, command);
                    config.visual_mode.insert(key, command)
                }
                _ => continue,
            };
        }

        Ok(config)
    }
}
