use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;

fn get_trigrams(s: &String) -> Vec<String> {
    let mut trigrams: Vec<String> = Vec::new();
    let mut p1 = ' ';
    let mut p2 = ' ';
    for c in s.chars() {
        let v = vec![p1, p2, c];
        let mut t: String = v.into_iter().collect();
        t = t.trim().to_owned();
        if t.len() == 3{
            trigrams.push(t);
        }
        p1 = p2;
        p2 = c;
    }
    trigrams
}

const punct :&'static str =  "!\"#$%&'()*+, -./:;<=>?@[\\]^_`{|}~\n";

struct CorpusAnalyzer {
    words: Vec<String>,
    pub weighted_bigrams: HashMap<String, f64>,
    pub occurrences: HashMap<String, HashMap<String, f64>>,
}

impl CorpusAnalyzer {  
    pub fn init(&self, corpus: Vec<String>) {
        self.words = corpus.into_iter().map(|word| word.trim_end().to_owned()).collect();
        self.init_weighted_bigrams();
        self.init_occurrences();
    }
    fn init_weighted_bigrams<'a>(&self) {
        let starting_bigrams: HashMap<String, f64> = HashMap::new();
        fn filtered_words(words: Vec<String>) -> Vec<String> {
            let mut filtered = Vec::new();
            for word in words {
                if word.len() > 2 && word[0..2].chars().all(|x| !punct.contains(x)) {
                    filtered.push(word);
                }
            }
            filtered
        }
        for word in filtered_words(self.words) {
            let bigram = word[0..2].to_uppercase();
            if starting_bigrams.iter().any(|(&x,&y)| x == bigram) {
                starting_bigrams[&bigram] += 1.0;
            } else {
                starting_bigrams[&bigram] = 1.0;
            }
        }
        let total: f64 = starting_bigrams.values().sum();
        for (bigram, count) in starting_bigrams.iter() {
            self.weighted_bigrams[bigram] = count / total;
        }
    }
    fn init_occurrences(&self) {
        let trigrams: Vec<(String, _)> = self.words.iter()
           .filter(|w| w.chars().all(char::is_alphanumeric))
            .flat_map(get_trigrams)
            .group_by(|x| x[0..2].to_string())
            .into_iter()
            .collect();

        let mut occurrences: HashMap<String, HashMap<String, f64>> = HashMap::new();
        
        for (bi,trigrams) in trigrams {
            let weights = occurrences.entry(bi.to_owned()).or_insert(HashMap::new());
            for tri in trigrams{
                let last_char = tri.chars().last().unwrap().to_string();
                weights.entry(last_char).and_modify(|e| { *e += 1.0 }).or_insert(1.0);
            }
        }

        for (bigram, last_letters) in occurrences.iter() {
            let total: f64 = last_letters.values().copied().sum();
            for last_letter in last_letters.keys() {
                occurrences[bigram][last_letter] /= total;
            }
        }
    }
}