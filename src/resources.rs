use std::{path::{Path, PathBuf}, fs, io::{ BufRead, BufReader, Result, Write }} ;

pub struct Resources {
    last_processed_id: Option<u16>,
    subscribed_topics: Option<Vec<String>>,
}

impl Resources {
    const BASE_PATH: &str = "~/.config/subscribe_hn";

    fn get_topics_store() -> PathBuf { Path::new(Resources::BASE_PATH).join("topics") }
    fn get_id_store() -> PathBuf { Path::new(Resources::BASE_PATH).join("last_processed_id") }

    pub fn fetch() -> Self {
        Resources {
            last_processed_id: Resources::fetch_id(),
            subscribed_topics: Resources::fetch_topics(), 
        }
    }

    pub fn update_id(id: u16) -> Result<()> {
        fs::write(Resources::get_id_store(), id.to_string())
    }

    pub fn upate_topic(topic: String) -> Result<usize> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(Resources::get_topics_store())?;

        let mut topic = String::from(topic);
        topic.push('\n');
        file.write( topic.as_bytes())
    }

    fn fetch_id() -> Option<u16> {
        let content = match fs::read_to_string(Resources::get_id_store()) {
            Ok(content) => Some(content),
            Err(_) => { None }
        };

        if let Some(content) = content {
            let id: u16 = content.trim().parse().unwrap_or(0);
            if id > 0 { return Some(id); }
        }

        None
    }

    fn fetch_topics() -> Option<Vec<String>> {
        match fs::File::open(Resources::get_topics_store()) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut reader_lines = reader.lines();
                let mut lines: Vec<String> = vec![];
                while let Some(line) = reader_lines.next() {
                    match line {
                        Ok(line) => lines.push(line),
                        Err(_) => continue
                    }
                };

                Some(lines)
            },
            Err(_) => None
        }
    }
}
