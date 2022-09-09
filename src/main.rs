pub mod regex;
use regex::Regex;
fn main() {
    let mut machine = Regex::new("114");
    // machine.init_current_state(&mut v);
    // machine.add2current_state(1, &mut v);
    println!(
        "Matching Result: {:?}",
        machine.bfs_match("abababbabababababcdedede")
    );
    println!(
        "Matching Result: {:?}",
        machine.bfs_match("abababbabababababccdedede")
    );
    println!(
        "Matching Result: {:?}",
        machine.bfs_match("ababgabbabababababcddedede")
    );
    println!(
        "Matching Result: {:?}",
        machine.bfs_match("ababgabbabababababcdeded")
    );
    println!(
        "Matching Result: {:?}",
        machine.bfs_match("ababbababaabbabababababc")
    );
    println!("Matching Result: {:?}", machine.bfs_match("c"));
}
