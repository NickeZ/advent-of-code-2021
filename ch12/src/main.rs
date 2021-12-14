use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Clone,PartialEq, Debug)]
enum NodeType {
    Large,
    Small,
    Start,
    End,
}

#[derive(Clone, Debug)]
struct Node {
    label: String,
    typ: NodeType,
}

impl Node {
    pub fn new(label: &str, typ: NodeType) -> Node {
        Node {label: label.to_string(), typ}
    }
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {nodes: Vec::new(), edges: Vec::new()}
    }

    pub fn add_node(&mut self, label: &str) {
        if self.node_by_label(label).is_none() {
            let typ = {
                if label == "start" {
                    NodeType::Start
                } else if label == "end" {
                    NodeType::End
                } else if label.chars().next().unwrap().is_uppercase() {
                    NodeType::Large
                } else {
                    NodeType::Small
                }
            };
            self.nodes.push(Node::new(label, typ));
            self.edges.push(Vec::new());
        }
    }

    pub fn add_edge(&mut self, label_a: &str, label_b: &str) {
        let node_a_id = self.node_by_label(label_a).unwrap();
        let node_b_id = self.node_by_label(label_b).unwrap();
        self.edges[node_a_id].push(node_b_id)
    }

    pub fn get_adjacent(&self, id: usize) -> &[usize] {
        &self.edges[id]
    }

    fn node_by_label(&self, label: &str) -> Option<usize> {
        self.nodes.iter().position(|s| s.label == label)
    }

    fn node(&self, id: usize) -> Node {
        self.nodes[id].clone()
    }
}

fn visit(g: &Graph, n: usize, history: &mut VecDeque<usize>, counter: &mut u32) {
    if g.node(n).label == "end" {
        *counter += 1;
        return
    }

    history.push_back(n);

    for adj in g.get_adjacent(n) {
        if !history.iter().any(|x|x == adj) || g.node(*adj).typ == NodeType::Large {
            visit(g, *adj, history, counter)
        }
    }

    history.pop_back();
}

fn visit2(g: &Graph, n: usize, history: &mut VecDeque<usize>, counter: &mut u32) {
    if g.node(n).label == "end" {
        *counter += 1;
        return
    }

    history.push_back(n);

    for adj in g.get_adjacent(n) {
        match g.node(*adj).typ {
            NodeType::Start => continue,
            NodeType::End  => {
                if history.iter().any(|x| x == adj) {
                    continue
                }
            },
            NodeType::Small => {
                let map = history.iter().filter(|x| g.node(**x).typ == NodeType::Small).fold(HashMap::new(), |mut map: HashMap<usize, u32>, x:&usize| {
                    let counter = map.entry(*x).or_insert(0);
                    *counter += 1;
                    map
                });
                if map.values().any(|v| *v == 2) && history.iter().any(|x| x == adj) {
                    continue
                }
            },
            _ => (),
        }
        visit2(g, *adj, history, counter);
    }

    history.pop_back();
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let mut g = Graph::new();
    for line in content.trim().split_whitespace() {
        let mut iter = line.split('-');
        let (a, b) = (iter.next().unwrap(), iter.next().unwrap());
        g.add_node(a);
        g.add_node(b);
        g.add_edge(a, b);
        g.add_edge(b, a);
    }

    let start = g.node_by_label("start").unwrap();

    {
        let mut history = std::collections::VecDeque::new();
        let mut counter = 0;
        visit(&g, start, &mut history, &mut counter);

        println!("{}", counter);
    }

    {
        let mut history = std::collections::VecDeque::new();
        let mut counter = 0;
        visit2(&g, start, &mut history, &mut counter);

        println!("{}", counter);
    }
}
