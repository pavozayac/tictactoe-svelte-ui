use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub player_one: String,
    pub player_two: String,
    pub depth: u8
}

impl Config {
    pub fn save(&self) -> std::io::Result<()> {
        let f = File::create("config.json")?;
        let writer = BufWriter::new(f);

        serde_json::to_writer_pretty(writer, &self)?;

        Ok(())
    }

    pub fn from_file() -> Config {
        let f = File::open("config.json");

        if let Ok(content) = f {
            let reader = BufReader::new(content);
            serde_json::from_reader(reader).unwrap()
        } else {
            Config {
                player_one: "".into(),
                player_two: "".into(),
                depth: 2
            }
        }
    }
}