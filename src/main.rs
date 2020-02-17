mod momblish;
fn main() {
    
    let momblish = momblish::Momblish::init("./corpus/corpus.json");
    for _ in 0..100000 {
        println!("{}",momblish.word(10));
    }
    
}
