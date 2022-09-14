pub mod regex;
use std::io;

use regex::Regex;
fn main() {
    println!("Your regex exp:");
    let mut regex_exp = String::new();
    io::stdin()
        .read_line(&mut regex_exp)
        .expect("failed to read line.");
    let mut machine = Regex::new(&regex_exp);

    loop {
        println!("Your string to match:");
        let mut be_matched = String::new();
        io::stdin()
            .read_line(&mut be_matched)
            .expect("failed to read line.");
        println!("Result: {}", machine.bfs_match(&be_matched));
    }
}
