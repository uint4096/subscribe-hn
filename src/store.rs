use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

pub trait Store<'a, T, U>
where
    T: FromStr,
{
    fn get_base_path() -> &'a str {
        "~/.config/subscribe_hn"
    }
    fn get_store() -> PathBuf;
    fn update(&mut self, elem: &T) -> ();
    fn fetch(&mut self) -> Option<U>;
}

pub struct ProcessedId(pub Option<u16>);
pub struct Topics(pub Option<Vec<String>>);

impl Store<'_, u16, u16> for ProcessedId {
    fn get_store() -> PathBuf {
        Path::new(ProcessedId::get_base_path()).join("last_processed_id")
    }

    fn update(&mut self, id: &u16) -> () {
        self.0 = Some(*id);
        let id = id.to_string();
        match fs::write(ProcessedId::get_store(), id) {
            Ok(()) => (),
            Err(e) => panic!("Error while writing id to disk! Error: {e}"),
        }
    }

    fn fetch(&mut self) -> Option<u16> {
        match self.0 {
            Some(id) => Some(id),
            None => {
                let content = match fs::read_to_string(ProcessedId::get_store()) {
                    Ok(content) => Some(content),
                    Err(_) => None,
                };

                if let Some(content) = content {
                    let id: u16 = content.trim().parse().unwrap_or(0);
                    if id > 0 {
                        return Some(id);
                    }
                }

                None
            }
        }
    }
}

impl Store<'_, String, Vec<String>> for Topics {
    fn get_store() -> PathBuf {
        Path::new(Topics::get_base_path()).join("topics")
    }

    fn update(&mut self, topic: &String) -> () {
        match fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(Topics::get_store())
        {
            Ok(mut file) => {
                let mut topic = String::from(topic);
                topic.push('\n');

                match file.write(&(topic.as_bytes())) {
                    Ok(_) => {
                        if let Some(mut topics) = self.0.to_owned() {
                            topics.push(topic);
                            self.0 = Some(topics);
                        } else {
                            self.0 = Some(vec![topic]);
                        }
                    }
                    Err(e) => {
                        panic!("Error while writing topics!. Error: {e}");
                    }
                }
            }
            Err(e) => {
                panic!("Error while opening file for writing!. Error: {e}")
            }
        }
    }

    fn fetch(&mut self) -> Option<Vec<String>> {
        match &self.0 {
            Some(topics) => Some(topics.to_owned()),
            None => {
                match fs::File::open(Topics::get_store()) {
                    Ok(file) => {
                        let reader = BufReader::new(file);
                        let mut reader_lines = reader.lines();
                        let mut lines: Vec<String> = vec![];

                        //todo: write an update many funtion
                        while let Some(line) = reader_lines.next() {
                            match line {
                                Ok(line) => {
                                    self.update(&line);
                                    lines.push(line);
                                }
                                Err(_) => continue,
                            }
                        }
                        Some(lines)
                    }
                    Err(_) => None,
                }
            }
        }
    }
}
