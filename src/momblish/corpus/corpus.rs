use std::collections::HashMap;
use std::env;
use std::fs;

pub struct Corpus<'a> {
    pub weighted_bigrams: HashMap<&'static str,f32>,
    pub occurrences: HashMap<&'static str,f32>,
}

impl Corpus <'_>{
    pub fn init<'a>(filepath: &str) -> &'static Corpus {
        Corpus::load(filepath)
    }
    fn load<'a>(path: &str) -> &'a Corpus {
        let data = fs::read_to_string(path).expect("Unable to read file");
        return json.loads(data);
    }

    // fn save<T0>(path: T0) {
    //     let saved_corpus = [
    //         ("weighted_bigrams", weighted_bigrams),
    //         ("occurrences", occurrences),
    //     ]
    //     .iter()
    //     .cloned()
    //     .collect::<HashMap<_, _>>();
    //     // with!(open(path, "w") as f) //unsupported
    //     {
    //         f.write(json.dumps(saved_corpus));
    //     }
    // }
}
