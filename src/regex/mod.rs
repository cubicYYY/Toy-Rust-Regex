// TODO: Using traits to refactor the part of state transformation,
// which allows it to handle different types of edges with some wrapper structs.
// It is mandatory work if advanced functions need to be implemented (for example,
// matching with a specific range of repeating times).
pub mod nfa;

use nfa::*;
use std::{cmp::Ordering, collections::HashMap};

pub type NfaNodeSet = Vec<NfaNodeHandle>;
trait BST<T, H>
where
    H: Copy,
{
    fn insert(&mut self, _x: &mut T) -> H {
        unimplemented!()
    }
    fn get(&mut self) -> H {
        unimplemented!()
    }
}
trait Splay<T, H>: BST<T, H>
where
    H: Copy,
{
    fn rotate(&mut self, x: H) -> ();
    fn splay(&mut self, x: H, y: Option<H>) -> ();
    fn insert(&mut self, x: &mut T) -> H;
    fn or_insert(&mut self, x: &mut T) -> H;
    fn get(&mut self, x: H) -> ();
}
pub type DfaStateHandle = usize;

#[derive(Debug)]
pub struct DfaBTreeState {
    nfa_nodes: NfaNodeSet,
    pub transform: HashMap<char, DfaStateHandle>,
    l_child: Option<DfaStateHandle>,
    r_child: Option<DfaStateHandle>,
    parent: Option<DfaStateHandle>,
}
#[derive(Debug)]
pub struct DfaBTree {
    pub states: Vec<Box<DfaBTreeState>>,
    pub root: Option<DfaStateHandle>,
}

impl DfaBTree {
    pub fn new() -> Self {
        Self {
            states: vec![],
            root: None,
        }
    }
}
impl<T, H> BST<T, H> for DfaBTree where H: Copy {}
impl DfaBTree {
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
}
impl Splay<NfaNodeSet, DfaStateHandle> for DfaBTree {
    fn rotate(&mut self, x: DfaStateHandle) {
        let parent = self.states[x].parent;
        let meta_parent = match parent {
            Some(fa) => self.states[fa].parent,
            None => None,
        };
        if let None = parent {
            return;
        }
        let parent = parent.unwrap();
        if let Some(meta_parent) = meta_parent {
            if self.states[meta_parent].l_child == Some(parent) {
                //zig
                self.states[meta_parent].l_child = Some(x);
            } else {
                //zag
                self.states[meta_parent].r_child = Some(x);
            }
            self.states[x].parent = Some(meta_parent);
        } else {
            self.states[x].parent = None;
        }

        if self.states[parent].l_child == Some(x) {
            //zig
            self.states[parent].l_child = self.states[x].r_child;
            if let Some(r) = self.states[x].r_child {
                self.states[r].parent = Some(parent);
            }
            self.states[x].r_child = Some(parent);
            self.states[parent].parent = Some(x);
        } else {
            //zag
            self.states[parent].r_child = self.states[x].l_child;
            if let Some(l) = self.states[x].l_child {
                self.states[l].parent = Some(parent);
            }
            self.states[x].l_child = Some(parent);
            self.states[parent].parent = Some(x);
        }
    }
    fn splay(&mut self, x: DfaStateHandle, target_parent: Option<DfaStateHandle>) {
        while self.states[x].parent != target_parent {
            let parent = self.states[x].parent.unwrap();
            // TODO check if unwrapping here is reasonable

            let meta_parent = self.states[parent].parent;
            if meta_parent != target_parent {
                let zigzag_flag = meta_parent.is_some()
                    && ((self.states[parent].l_child == Some(x))
                        ^ (self.states[meta_parent.unwrap()].l_child == Some(parent)));
                if zigzag_flag {
                    self.rotate(x);
                } else {
                    self.rotate(parent);
                }
            }
            self.rotate(x);
        }
        if let None = self.root {
            self.root = Some(x);
        }
    }
    fn insert(&mut self, _list: &mut NfaNodeSet) -> DfaStateHandle {
        unimplemented!()
    }
    fn or_insert(&mut self, list: &mut NfaNodeSet) -> DfaStateHandle {
        // TODO refactor.
        let mut cur = self.root;
        let mut last = None;
        let mut select = Ordering::Equal;
        let is_found = 'search_tree: loop {
            if let None = cur {
                break 'search_tree false;
            }
            let cur_handle = cur.unwrap();
            select = DfaBTree::list_cmp(&self.states[cur_handle].nfa_nodes, list);
            last = cur;
            cur = match select {
                Ordering::Greater => self.states[cur_handle].l_child,
                Ordering::Less => self.states[cur_handle].r_child,
                Ordering::Equal => break 'search_tree true,
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
            parent: None,
        }));

        match select {
            Ordering::Greater => {
                let last = last.unwrap();
                assert!(self.states[last].l_child.is_none());
                self.states[last].l_child = Some(handle);
                self.states[handle].parent = Some(last);
            }
            Ordering::Less => {
                let last = last.unwrap();
                assert!(self.states[last].r_child.is_none());
                self.states[last].r_child = Some(handle);
                self.states[handle].parent = Some(last);
            }
            Ordering::Equal => {
                assert!(self.root.is_none());
                self.root = Some(handle);
            }
        };
        assert!(self.root.is_some());
        self.splay(handle, None);
        handle
    }
    fn get(&mut self, _dnode: DfaStateHandle) {
        unimplemented!()
    }
}
pub struct Regex {
    nfa: Nfas,
    dfa: DfaBTree,
    begin: NfaNodeHandle,
    end: NfaNodeHandle,
    last_update: usize,
}

impl Regex {
    pub fn new(regex_str: &str) -> Self {
        let mut graph = Self {
            nfa: Nfas::new(),
            dfa: DfaBTree::new(),
            begin: 0,
            end: 0,
            last_update: 1,
        };
        let NfaPaired { start, end } = graph.nfa.build_nfa(regex_str);
        (graph.begin, graph.end) = (start, end);
        graph
    }

    pub fn add2current_state(&mut self, node: NfaNodeHandle, list: &mut NfaNodeSet) -> () {
        let mut dfs_stack: NfaNodeSet = vec![];
        if self.nfa.nodes[node].id == self.last_update {
            return;
        }
        self.nfa.nodes[node].id = self.last_update;
        dfs_stack.push(node);
        while !dfs_stack.is_empty() {
            let cur = dfs_stack.pop().unwrap();
            list.push(cur);
            if let Some(Edge {
                to: target,
                trans: Transform::Epsilon,
            }) = self.nfa.nodes[cur].edge1
            {
                if self.nfa.nodes[target].id != self.last_update {
                    self.nfa.nodes[target].id = self.last_update;
                    dfs_stack.push(target);
                }
            }
            if let Some(Edge {
                to: target,
                trans: Transform::Epsilon,
            }) = self.nfa.nodes[cur].edge2
            {
                if self.nfa.nodes[target].id != self.last_update {
                    self.nfa.nodes[target].id = self.last_update;
                    dfs_stack.push(target);
                }
            }
        }
    }

    pub fn get_dfa_state(&mut self, list: &mut NfaNodeSet) -> DfaStateHandle {
        list.sort();
        self.dfa.or_insert(list)
    }

    pub fn step(&mut self, old_list: &mut NfaNodeSet, chr: char) -> DfaStateHandle {
        let mut new_list: NfaNodeSet = vec![];

        for &node in old_list.iter() {
            if let Some(Edge {
                to: target,
                trans: Transform::Trans(edge_ch),
            }) = self.nfa.nodes[node].edge1
            {
                if chr == edge_ch {
                    self.add2current_state(target, &mut new_list);
                }
            }
            if let Some(Edge {
                to: target,
                trans: Transform::Trans(edge_ch),
            }) = self.nfa.nodes[node].edge2
            {
                if chr == edge_ch {
                    self.add2current_state(target, &mut new_list);
                }
            }
        }
        self.get_dfa_state(&mut new_list)
    }
    pub fn get_next_state(&mut self, dstate: DfaStateHandle, chr: char) -> DfaStateHandle {
        self.last_update += 1;
        match self.dfa.states[dstate].transform.get(&chr) {
            Some(&next_state) => next_state,
            None => {
                let mut old_states = self.dfa.states[dstate].nfa_nodes.clone();
                self.step(&mut old_states, chr)
            }
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
        }
        // dbg!(&self.dfa.states);
        for &nfa_node in self.dfa.states[cur_state].nfa_nodes.iter() {
            if self.nfa.nodes[nfa_node].is_end {
                return true;
            }
        }

        false
    }
}
