mod momblish;
fn main() {
    
    let momblish = momblish::Momblish::init("./corpus/corpus.json");
    
    println!("{}",momblish.word(10));
}
