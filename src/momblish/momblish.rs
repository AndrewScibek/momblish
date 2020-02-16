use std::collections::HashMap;
use std::process;
extern crate rand;
mod corpus;
use rand::Rng;

pub struct Momblish<'a> {
    corpus: &'static corpus::Corpus,
}

impl Momblish{
    pub fn init<'a>(path: &'static str) -> &'static Momblish {
        let corpus = corpus::Corpus::init(path);
        return &Momblish { corpus }
    }
    fn word<'a>(self, length: u32) -> &'a str {
        let mut word = random.choices(
            self.corpus.weighted_bigrams.keys().collect::<Vec<_>>(),
            self.corpus.weighted_bigrams.values(),
        )[0];
        for _ in 0..length - 2 {
            let last_bigram = word[-2..];
            let next_letter = random.choices(
                self.corpus.occurrences[last_bigram].collect::<Vec<_>>(),
                self.corpus.occurrences[last_bigram].values(),
            )[0];
            word += next_letter;
        }
        return word;
    }
    fn sentence<'a>(self, count: u32, word_length: u32) -> &'a str {
        let mut out = "".to_string();
        for _ in 0..count {
            out.push_str( self.word(word_length));
        }
        return out.as_str();
    }
}
