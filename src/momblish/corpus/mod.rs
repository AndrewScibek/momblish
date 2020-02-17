use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Corpus{
    pub weighted_bigrams: HashMap<String, f64>,
    pub occurences: HashMap<String, HashMap<String, f64>>,
}

impl Corpus {
    pub fn init(filepath: &str) -> Corpus {
        Corpus::load(filepath)
    }
    fn load(path: &str) -> Corpus {
        let _data = fs::read_to_string(path).expect("Unable to read file");
        let deserialized: Corpus = serde_json::from_str(&_data).unwrap();
        return deserialized;
    }
}