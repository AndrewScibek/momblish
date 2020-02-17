mod momblish;
fn main() {
    
    let momblish = momblish::Momblish::init("./corpus/corpus.json");
    for _ in 0..10000000 {
        momblish.word(10);
    }
}
