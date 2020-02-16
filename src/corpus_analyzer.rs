use std::collections::HashMap;
use std::*;

use collections::defaultdict;
use itertools::{chain, groupby};
use momblish::corpus::Corpus;
fn each_cons<T0, T1, RT>(xs: T0, n: T1) -> RT {
    return zip(
        starred!(0..n.iter().map(|i| xs[i..]).collect::<Vec<_>>()), /*unsupported*/
    );
}
struct CorpusAnalyzer {
    words: ST0,
    corpus: ST1,
}

impl CorpusAnalyzer {
    fn __init__<T0>(&self, corpus: T0) {
        self.words = corpus.iter().map(|word| word.rstrip()).collect::<Vec<_>>();
        self.corpus = Corpus(HashMap::new(), HashMap::new());
        self.init_weighted_bigrams();
        self.init_occurrences();
    }
    fn init_weighted_bigrams(&self) {
        let starting_bigrams = HashMap::new();
        fn filtered_words() {
            for word in self.words {
                if word.len() > 2 && word[0..2].iter().all(|&x| x != string.punctuation) {
                    //yield is unimplemented;
                }
            }
        }
        for word in filtered_words() {
            let bigram = word[0..2].upper();
            if starting_bigrams.iter().any(|&x| x == bigram) {
                starting_bigrams[bigram] += 1;
            } else {
                starting_bigrams[bigram] = 1;
            }
        }
        let total = starting_bigrams.values().iter().sum();
        for (bigram, count) in starting_bigrams.items() {
            self.corpus.weighted_bigrams[bigram] = count / total;
        }
    }
    fn init_occurrences(&self) {
        let punct_and_newline = set(string.punctuation
            + "
");
        let mut trigrams = vec![];
        for word in self.words {
            if punct_and_newline.isdisjoint(set(word)) {
                trigrams.push(each_cons(word.upper(), 3).collect::<Vec<_>>());
            }
        }
        trigrams = chain::from_iterable(trigrams).collect::<Vec<_>>();
        self.corpus.occurrences = defaultdict(dict);
        for (bigram, trigram) in groupby(trigrams, |x| "".join(x[0..2])) {
            let last_char = trigram.collect::<Vec<_>>()[0][-1];
            if self.corpus.occurrences[bigram]
                .iter()
                .any(|&x| x == last_char)
            {
                self.corpus.occurrences[bigram][last_char] += 1;
            } else {
                self.corpus.occurrences[bigram][last_char] = 1;
            }
        }
        for (bigram, last_letters) in self.corpus.occurrences.items() {
            let total = last_letters.values().iter().sum();
            for last_letter in last_letters {
                self.corpus.occurrences[bigram][last_letter] /= total;
            }
        }
    }
}
