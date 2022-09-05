pub type NfaNodeHandle = usize;

#[derive(Debug, Clone, Copy)]
pub enum Transform {
    Trans(char),
    Epsilon,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub to: NfaNodeHandle,
    pub trans: Transform,
}
impl Edge {
    pub fn new(to: NfaNodeHandle, trans: Transform) -> Self {
        Self {
            to: to,
            trans: trans,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NfaNode {
    pub is_end: bool,
    pub edge1: Option<Edge>,
    pub edge2: Option<Edge>,
    pub id: usize, //unique id for each state update step
}
impl NfaNode {
    pub fn new(is_end: bool, edge1: Option<Edge>, edge2: Option<Edge>) -> NfaNode {
        Self {
            is_end: is_end,
            edge1: edge1,
            edge2: edge2,
            id: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NfaPaired {
    pub start: NfaNodeHandle,
    pub end: NfaNodeHandle,
}

#[derive(Debug)]
pub struct Nfas {
    pub nodes: Vec<NfaNode>,
    recycle: Vec<NfaNodeHandle>,
}

// struct for non-deterministic finite automata
// the Thompson's constructtion, where every node has less than 2 out degrees.
impl Nfas {
    pub fn new() -> Nfas {
        Self {
            nodes: vec![],
            recycle: vec![],
        }
    }

    fn new_node(&mut self, is_end: bool) -> NfaNodeHandle {
        let handle: usize;
        if !self.recycle.is_empty() {
            handle = self.recycle.pop().unwrap();
            self.nodes[handle] = NfaNode::new(is_end, None, None);
        } else {
            handle = self.nodes.len();
            self.nodes.push(NfaNode::new(is_end, None, None));
        };
        handle
    }

    pub fn new_unit(&mut self, trans: Transform) -> NfaPaired {
        // basic component in the Thompson's construction
        let start = self.new_node(false);
        let end = self.new_node(true);
        self.nodes[start].edge1 = Some(Edge::new(end, trans));
        NfaPaired {
            start: start,
            end: end,
        }
    }

    pub fn union(&mut self, nfa1: NfaPaired, nfa2: NfaPaired) -> NfaPaired {
        // union the patterns (|)
        let start = self.new_node(false);
        let end = self.new_node(true);
        self.nodes[start].edge1 = Some(Edge::new(nfa1.start, Transform::Epsilon));
        self.nodes[start].edge2 = Some(Edge::new(nfa2.start, Transform::Epsilon));
        self.nodes[nfa1.end].edge1 = Some(Edge::new(end, Transform::Epsilon));
        self.nodes[nfa2.end].edge1 = Some(Edge::new(end, Transform::Epsilon));

        self.nodes[nfa1.end].is_end = false;
        self.nodes[nfa2.end].is_end = false;
        self.nodes[end].is_end = true;
        NfaPaired {
            start: start,
            end: end,
        }
    }

    pub fn concat(&mut self, nfa1: NfaPaired, nfa2: NfaPaired) -> NfaPaired {
        // concat the pattern
        self.nodes[nfa1.end] = self.nodes[nfa2.start];
        self.recycle.push(nfa2.start);
        // TODO: recycle the node properly to avoid potential illegal accesses
        self.nodes[nfa1.end].is_end = false; // Actually useless...

        NfaPaired {
            start: nfa1.start,
            end: nfa2.end,
        }
    }

    pub fn into_positive(&mut self, nfa: NfaPaired) -> NfaPaired {
        // construct a positive closure (+)
        let start = self.new_node(false);
        let end = self.new_node(true);

        self.nodes[start].edge1 = Some(Edge::new(nfa.start, Transform::Epsilon));
        self.nodes[nfa.end].edge1 = Some(Edge::new(nfa.start, Transform::Epsilon));
        self.nodes[nfa.end].edge2 = Some(Edge::new(end, Transform::Epsilon));

        self.nodes[nfa.end].is_end = false;

        NfaPaired {
            start: start,
            end: end,
        }
    }

    pub fn into_kleene(&mut self, nfa: NfaPaired) -> NfaPaired {
        // construct a kleene closure (*)
        // The only difference to positive closure is that it added a link from start node to end node
        // which allows it to skip the pattern inside
        let nfa = self.into_positive(nfa);
        self.nodes[nfa.start].edge2 = Some(Edge::new(nfa.end, Transform::Epsilon));

        NfaPaired {
            start: nfa.start,
            end: nfa.end,
        }
    }

    pub fn into_option(&mut self, nfa: NfaPaired) -> NfaPaired {
        // construct a option closure (?)
        let start = self.new_node(false);
        let end = self.new_node(true);

        self.nodes[start].edge1 = Some(Edge::new(nfa.start, Transform::Epsilon));
        self.nodes[start].edge1 = Some(Edge::new(end, Transform::Epsilon));
        self.nodes[nfa.end].edge1 = Some(Edge::new(end, Transform::Epsilon));

        self.nodes[nfa.end].is_end = false;

        NfaPaired {
            start: start,
            end: end,
        }
    }

    pub fn build_nfa(&mut self, regex_str: &str) -> NfaPaired {
        let x = self.new_unit(Transform::Trans('a'));
        self.into_kleene(x)
        // only for test

        // TODO: build NFA from a given regex expression
    }
}
