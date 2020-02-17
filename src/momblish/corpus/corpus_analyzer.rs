// use std::collections::HashMap;

// fn each_cons<T0, T1, RT>(xs: T0, n: T1) -> RT {
//     return zip(
//         starred!(0..n.iter().map(|i| xs[i..]).collect::<Vec<_>>()), /*unsupported*/
//     );
// }

// struct CorpusAnalyzer<'a> {
//     words: Vec<&'a str>,
//     pub weighted_bigrams: HashMap<&'static str, f32>,
//     pub occurrences: HashMap<&'static str, HashMap<&'static str, f32>>,
// }

// impl CorpusAnalyzer<'_> {
//     pub fn init<'a>(&self, corpus: Vec<&'a str>) {
//         self.words = corpus.iter().map(|word| word.rstrip()).collect::<Vec<_>>();
//         self.init_weighted_bigrams();
//         self.init_occurrences();
//     }
//     fn init_weighted_bigrams<'a>(&self) {
//         let starting_bigrams = HashMap::new();
//         fn filtered_words(words: Vec<&str>) -> Vec<&str> {
//             let mut filtered = Vec::new();
//             for word in words {
//                 if word.len() > 2 && word[0..2].iter().all(|&x| x != string.punctuation) {
//                     filtered.push(word);
//                 }
//             }
//             return filtered;
//         }
//         for word in filtered_words(self.words) {
//             let bigram = word[0..2].upper();
//             if starting_bigrams.iter().any(|&x| x == bigram) {
//                 starting_bigrams[bigram] += 1;
//             } else {
//                 starting_bigrams[bigram] = 1;
//             }
//         }
//         let total = starting_bigrams.values().iter().sum();
//         for (bigram, count) in starting_bigrams.items() {
//             self.weighted_bigrams[bigram] = count / total;
//         }
//     }
//     fn init_occurrences<'a>(&self) {
//         let punct_and_newline = set(string.punctuation
//             + "
// ");
//         let mut trigrams = vec![];
//         for word in self.words {
//             if punct_and_newline.isdisjoint(set(word)) {
//                 trigrams.push(each_cons(word.upper(), 3).collect::<Vec<_>>());
//             }
//         }
//         trigrams = chain::from_iterable(trigrams).collect::<Vec<_>>();
//         for (bigram, trigram) in groupby(trigrams, |x| "".join(x[0..2])) {
//             let last_char = trigram.collect::<Vec<_>>()[0][-1];
//             if self.occurrences[bigram]
//                 .iter()
//                 .any(|&x| x == last_char)
//             {
//                 self.occurrences[bigram][last_char] += 1.0;
//             } else {
//                 self.occurrences[bigram][last_char] = 1.0;
//             }
//         }
//         for (bigram, last_letters) in self.occurrences.items() {
//             let total = last_letters.values().iter().sum();
//             for last_letter in last_letters {
//                 self.occurrences[bigram][last_letter] /= total;
//             }
//         }
//     }
// }
