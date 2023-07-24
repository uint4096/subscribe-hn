use std::{
    fs,
    io::{BufRead, BufReader, Result, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use dirs;

pub trait Store<'a, T, U>
where
    T: FromStr,
{
    fn get_base_path() -> String {
        let home_dir = match dirs::home_dir() {
            Some(path) => {
                match path.into_os_string().into_string() {
                    Ok(path) => path,
                    Err(_) => panic!("Failed to resolve path!")
                }
            }
            None => { panic!("Unable to find home directory!") } 
        };
        
        format!("{}/.config/subscribe_hn", home_dir)
    }

    fn new(elem: Option<U>) -> Self;
    fn get_store() -> PathBuf;
    fn update(&mut self, elem: &T) -> ();
    fn delete(&mut self, _: &T) -> () {
        ()
    }
    fn overwrite(&mut self, _: U) -> Result<()> {
        Ok(())
    }
    fn fetch(&mut self) -> Option<U>;
}

pub struct ProcessedId(Option<u32>);

impl Store<'_, u32, u32> for ProcessedId {
    fn get_store() -> PathBuf {
        Path::new(&ProcessedId::get_base_path()).join("last_processed_id")
    }

    fn new(id: Option<u32>) -> Self {
        let store_path = ProcessedId::get_base_path();
        fs::create_dir_all(store_path).expect("Error while creating the store!");
        ProcessedId(id)
    }

    fn update(&mut self, id: &u32) -> () {
        self.0 = Some(*id);
        let id = id.to_string();
        fs::write(ProcessedId::get_store(), id).expect("Error while writing id to disk!")
    }

    fn fetch(&mut self) -> Option<u32> {
        match self.0 {
            Some(id) => Some(id),
            None => {
                let content = match fs::read_to_string(ProcessedId::get_store()) {
                    Ok(content) => Some(content),
                    Err(_) => None,
                };

                if let Some(content) = content {
                    let id: u32 = content.trim().parse().unwrap_or(0);
                    if id > 0 {
                        return Some(id);
                    }
                }

                None
            }
        }
    }
}

#[derive(Clone)]
pub struct Topics(Option<Vec<String>>);

impl Store<'_, String, Vec<String>> for Topics {
    fn get_store() -> PathBuf {
        Path::new(&Topics::get_base_path()).join("topics")
    }

    fn new(topics: Option<Vec<String>>) -> Self {
        let store_path = ProcessedId::get_base_path();
        fs::create_dir_all(store_path).expect("Error while creating the store!");
        Topics(topics)
    }

    fn delete(&mut self, topic: &String) -> () {
        if let Some(mut elems) = self.fetch() {
            elems.retain(|e| e != &topic.to_lowercase());
            match self.overwrite(elems) {
                Ok(_) => (),
                Err(e) => panic!("Error while overwriting topics! Error: {e}"),
            }
        };
    }

    fn overwrite(&mut self, topics: Vec<String>) -> Result<()> {
        self.0 = Some(topics.clone());
        fs::write(Topics::get_store(), format!("{}\n", topics.join("\n")))
    }

    fn update(&mut self, topic: &String) -> () {
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(Topics::get_store())
        {
            Ok(mut file) => {
                let mut topic = String::from(topic);
                topic.push('\n');

                match file.write(&(topic.as_bytes())) {
                    Ok(_) => {
                        if let Some(mut topics) = self.fetch() {
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
                                    lines.push(line);
                                }
                                Err(_) => continue,
                            }
                        }

                        self.0 = Some(lines.clone());
                        Some(lines)
                    }
                    Err(_) => None,
                }
            }
        }
    }
}

pub struct SentStories;
impl Store<'_, String, Vec<String>> for SentStories {
    fn get_store() -> PathBuf {
        Path::new(&Topics::get_base_path()).join("sent_stories")
    }

    fn new(_: Option<Vec<String>>) -> Self {
        let store_path = SentStories::get_base_path();
        fs::create_dir_all(store_path).expect("Error while creating the store!");
        SentStories
    }

    fn fetch(&mut self) -> Option<Vec<String>> {
        match fs::File::open(SentStories::get_store()) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut reader_lines = reader.lines();
                let mut lines: Vec<String> = vec![];

                while let Some(line) = reader_lines.next() {
                    match line {
                        Ok(line) => {
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

    fn update(&mut self, title: &String) -> () {
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(SentStories::get_store())
        {
            Ok(mut file) => {
                let mut title = String::from(title);
                title.push('\n');

                if let Err(e) = file.write(&(title.as_bytes())) {
                    panic!("Error while writing sent stories!. Error: {e}");
                }
            }
            Err(e) => {
                panic!("Error while opening file for writing!. Error: {e}")
            }
        }
    }
}
