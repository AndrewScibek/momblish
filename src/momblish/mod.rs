mod corpus;
use random_choice::random_choice;
pub struct Momblish {
    corpus: corpus::Corpus,
    choices: Vec<String>,
    weights: Vec<f64>,
}
impl Momblish {
    pub fn init(path: &'static str) -> Momblish {
        let corpus = corpus::Corpus::init(path);
        let choices = corpus.weighted_bigrams.keys().cloned().collect();
        let weights = corpus.weighted_bigrams.values().copied().collect();
        Momblish { corpus, choices, weights }
    }
    pub fn word(&self, length: usize) -> String {
        let mut rng = random_choice();
        let mut buf = String::with_capacity(length);
        let word: &str = rng.random_choice_f64(&self.choices, &self.weights, 1)[0];
        buf.push_str(word);
        for _ in 0..length - 2 {
            let len = buf.len();
            let last_bigram = &buf[len-2..];
            let choices: Vec<&String> = self.corpus.occurrences[last_bigram].keys().collect::<Vec<_>>();
            let weights: Vec<f64> = self.corpus.occurrences[last_bigram].values().copied().collect();
            let next_letter = rng.random_choice_f64(&choices, &weights, 1)[0];
            buf.push_str(next_letter);
        }
        buf
    }
    pub fn sentence(&self, count: u32, word_length: usize) -> String {
        let mut out = "".to_string();
        for _ in 0..count {
            out.push_str(self.word(word_length).as_str());
            out.push_str(" ");
        }
        return out.to_owned();
    }
}