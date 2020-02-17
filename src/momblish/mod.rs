mod corpus;
use random_choice::random_choice;

pub struct Momblish {
    corpus: corpus::Corpus,
}
impl Momblish {
    pub fn init(path: &'static str) -> Momblish {
        let _corpus = corpus::Corpus::init(path);
        return Momblish { corpus: _corpus };
    }
    pub fn word<'a>(&self, _length: u32) -> String {
        let mut choices: Vec<&String> = self.corpus.weighted_bigrams.keys().collect::<Vec<_>>();
        let mut weights: Vec<f64> = self.corpus.weighted_bigrams.values().map(|percent| *percent).collect::<Vec<_>>();
        let word: &str = random_choice().random_choice_f64(&choices, &weights, 1)[0];
        let mut owned_word = word.to_owned();
        for _ in 0.._length - 2 {
            let len = owned_word.len();
            let last_bigram = &owned_word[len-2..];
            choices = self.corpus.occurrences[last_bigram].keys().collect::<Vec<_>>();
            weights = self.corpus.occurrences[last_bigram].values().map(|percent| *percent).collect::<Vec<_>>();
            let next_letter: &str = random_choice().random_choice_f64(&choices, &weights, 1)[0];
            owned_word.push_str(next_letter);
        }
        return owned_word;
    }
    pub fn sentence<'a>(&self, count: u32, word_length: u32) -> String {
        let mut out = "".to_string();
        for _ in 0..count {
            out.push_str(self.word(word_length).as_str());
            out.push_str(" ");
        }
        return out.to_owned();
    }
}