use std::collections::HashMap;
use std::*;

struct Corpus {
    weighted_bigrams: ST0,
    occurrences: ST1,
}

impl Corpus {
    fn __init__<T0, T1>(&self, weighted_bigrams: T0, occurrences: T1) {
        self.weighted_bigrams = weighted_bigrams;
        self.occurrences = occurrences;
    }
    fn load<T0, T1, RT>(cls: T0, path: T1) -> RT {
        // with!(open(path, "r") as f) //unsupported
        {
            let data = f.read();
        }
        return cls(json.loads(data));
    }
    fn __eq__<T0, RT>(&self, other: T0) -> RT {
        return self.weighted_bigrams == other.weighted_bigrams
            && self.occurrences == other.occurrences;
    }
    fn save<T0>(&self, path: T0) {
        let saved_corpus = [
            ("weighted_bigrams", self.weighted_bigrams),
            ("occurrences", self.occurrences),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        // with!(open(path, "w") as f) //unsupported
        {
            f.write(json.dumps(saved_corpus));
        }
    }
}
