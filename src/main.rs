mod momblish;
fn main() {
    
    let momblish = momblish::Momblish::init("./corpus/corpus.json");
    for _ in 0..1000 {
        println!("{}",momblish.word(8));
    }
    
}
