mod momblish;
mod corpus;
use rand::thread_rng;
mod corpus_analyzer;
fn main() {
    
    let momblish = momblish::Momblish::init("./corpus/corpus.json");
    let rng = thread_rng();
    for _ in 0..10_000_000 {
       momblish.word(10,rng.clone());
    }
}
