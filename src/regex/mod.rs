// TODO: Using traits to refactor the part of state transformation,
// which allows it to handle different types of edges with some wrapper structs.
// It is mandatory work if advanced functions need to be implemented (for example, 
// matching with a specific range of repeating times).
pub mod nfa;

use nfa::*;
use std::{cmp::Ordering, collections::HashMap};

pub type NfaNodeSet = Vec<NfaNodeHandle>;

pub fn list_cmp(list1: &NfaNodeSet, list2: &NfaNodeSet) -> Ordering {
    if list1.len() > list2.len() {
        return Ordering::Greater;
    } else if list1.len() < list2.len() {
        return Ordering::Less;
    }
    for (node1, node2) in list1.iter().zip(list2.iter()) {
        if *node1 > *node2 {
            return Ordering::Greater;
        } else if *node1 < *node2 {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

pub type DfaStateHandle = usize;

#[derive(Debug)]
pub struct DfaBTreeState {
    nfa_nodes: NfaNodeSet,
    pub transform: HashMap<char, DfaStateHandle>,
    l_child: Option<DfaStateHandle>,
    r_child: Option<DfaStateHandle>,
}

pub struct Regex {
    fa: Nfas,
    states_root: Option<DfaStateHandle>,
    pub states: Vec<Box<DfaBTreeState>>,
    begin: NfaNodeHandle,
    end: NfaNodeHandle,
    last_update: usize,
}

impl Regex {
    pub fn new(regex_str: &str) -> Self {
        let mut graph = Self {
            fa: Nfas::new(),
            states_root: None,
            states: vec![],
            begin: 0,
            end: 0,
            last_update: 1,
        };
        let NfaPaired { start, end } = graph.fa.build_nfa(regex_str);
        (graph.begin, graph.end) = (start, end);
        graph
    }

    pub fn add2current_state(&mut self, node: NfaNodeHandle, list: &mut NfaNodeSet) -> () {
        let mut dfs_stack: NfaNodeSet = vec![];
        if self.fa.nodes[node].id == self.last_update {
            return;
        }
        self.fa.nodes[node].id = self.last_update;
        dfs_stack.push(node);
        while !dfs_stack.is_empty() {
            let cur = dfs_stack.pop().unwrap();
            list.push(cur);
            if let Some(Edge {
                to: target,
                trans: Transform::Epsilon,
            }) = self.fa.nodes[cur].edge1
            {
                if self.fa.nodes[target].id != self.last_update {
                    self.fa.nodes[target].id = self.last_update;
                    dfs_stack.push(target);
                }
            }
            if let Some(Edge {
                to: target,
                trans: Transform::Epsilon,
            }) = self.fa.nodes[cur].edge2
            {
                if self.fa.nodes[target].id != self.last_update {
                    self.fa.nodes[target].id = self.last_update;
                    dfs_stack.push(target);
                }
            }
        }
    }

    pub fn get_dfa_state(&mut self, list: &mut NfaNodeSet) -> DfaStateHandle {
        list.sort();
        let mut cur: Option<DfaStateHandle> = self.states_root;
        let mut last = None;
        let mut select = Ordering::Equal;

        let is_found = 'down_the_tree: loop {
            if let None = cur {
                break 'down_the_tree false;
            }
            select = super::list_cmp(&self.states[cur.unwrap()].nfa_nodes, list);
            last = cur;
            cur = match select {
                Ordering::Greater => self.states[cur.unwrap()].l_child,
                Ordering::Less => self.states[cur.unwrap()].r_child,
                Ordering::Equal => break 'down_the_tree true,
            };
        };

        if is_found {
            return last.unwrap();
        }

        let handle = self.states.len();
        self.states.push(Box::new(DfaBTreeState {
            nfa_nodes: list.clone(),
            transform: HashMap::new(),
            l_child: None,
            r_child: None,
        }));
        match select {
            Ordering::Greater => {
                assert!(self.states[last.unwrap()].l_child.is_none());
                self.states[last.unwrap()].l_child = Some(handle)
            }
            Ordering::Less => {
                assert!(self.states[last.unwrap()].r_child.is_none());
                self.states[last.unwrap()].r_child = Some(handle)
            }
            Ordering::Equal => {
                assert!(self.states_root.is_none());
                self.states_root = Some(handle);
            }
        };
        handle
    }

    pub fn step(&mut self, old_list: &mut NfaNodeSet, chr: char) -> DfaStateHandle {
        let mut new_list: NfaNodeSet = vec![];

        for &node in old_list.iter() {
            if let Some(Edge { to: target, trans: Transform::Trans(edge_ch) }) = self.fa.nodes[node].edge1 {
                if chr == edge_ch {
                    self.add2current_state(target, &mut new_list);
                }
            }
            if let Some(Edge { to: target, trans: Transform::Trans(edge_ch) }) = self.fa.nodes[node].edge2 {
                if chr == edge_ch {
                    self.add2current_state(target, &mut new_list);
                }
            }
        }
        self.get_dfa_state(&mut new_list)
    }
    pub fn get_next_state(&mut self, dstate: DfaStateHandle, chr: char) -> DfaStateHandle {
        self.last_update += 1;
        match self.states[dstate].transform.get(&chr) {
            Some(&next_state) => {
                next_state
            },
            None => {
                let mut old_states = self.states[dstate].nfa_nodes.clone();
                self.step(&mut old_states, chr)
            },
        }
    }
    pub fn init_current_state(&mut self, list: &mut NfaNodeSet) -> DfaStateHandle {
        self.last_update += 1;
        list.clear();
        self.add2current_state(self.begin, list);
        self.get_dfa_state(list)
    }

    pub fn bfs_match(&mut self, query: &str) -> bool {
        // O(mn)

        // First, get the initial state list
        let mut nodes: NfaNodeSet = vec![];
        let mut cur_state = self.init_current_state(&mut nodes);
        for ch in query.chars() {
            cur_state = self.get_next_state(cur_state, ch);
            println!("{:?}", self.states);
        }
        for &nfa_node in self.states[cur_state].nfa_nodes.iter() {
            if self.fa.nodes[nfa_node].is_end {
                return true;
            }
        }
        false
    }
}
