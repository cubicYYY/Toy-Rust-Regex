pub mod regex;
use regex::nfa::{Nfas, Transform};
use regex::Regex;
use regex::list_cmp;

use crate::regex::DfaBTreeState;
use crate::regex::nfa::NfaNodeHandle;
fn main() {
    let mut machine = Regex::new("114");
    let mut v: Vec<NfaNodeHandle>= vec![1,2,3];
    // TODO: using Balanced Tree to store the list
    // machine.init_current_state(&mut v);
    // machine.add2current_state(1, &mut v);
    println!("{:?}", machine.bfs_match(""));

}
