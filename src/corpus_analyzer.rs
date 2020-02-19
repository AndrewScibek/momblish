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
    fn init_occurrences<'a>(&self) {
        
        let punct_and_newline = HashSet::from_iter(punct.as_bytes().iter().cloned());
        let mut trigrams: Vec<String> = Vec::new();
        for word in self.words {
            if punct_and_newline.is_disjoint(&HashSet::from_iter(word.as_bytes().iter().cloned())) {
                trigrams.extend(get_trigrams(&word.to_uppercase()));
            }
        }
        let collection: HashMap<String, Vec<String>> = trigrams.iter()
                .group_by(|x| &x[0..2]).into_iter()
                .map(|(ge0, group)| (ge0, group.cloned().collect()))
                .collect();
        for (bigram, trigram) in  collection{
            let last_char = &trigram[0].chars().last().unwrap().to_string();
            if self.occurrences[&bigram]
                .iter()
                .any(|(x,&_)| x == last_char)
            {
                self.occurrences[&bigram][last_char] += 1.0;
            } else {
                self.occurrences[&bigram][last_char] = 1.0;
            }
        }
        for (bigram, last_letters) in self.occurrences.iter() {
            let total = last_letters.values().copied().sum();
            for last_letter in last_letters.keys() {
                self.occurrences[bigram][last_letter] /= total;
            }
        }
    }
}
