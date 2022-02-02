fn main() {
    
    let the_string = String::from("youtttube");
    let len = the_string.len();
    for s in (0..len).enumerate() {
        println!("{}", the_string[s])
    }

    // println!("{}", len);
}
