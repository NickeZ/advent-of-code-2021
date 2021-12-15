use std::fs::File;
use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Node {
    x: usize,
    y: usize,
    cost: u32,
}

impl Node {
    fn new(x: usize, y: usize, cost: u32) -> Node {
        Node {x, y, cost}
    }
}

struct Graph {
    width: u32,
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {width: 0, nodes: Vec::new(), edges: Vec::new() }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.edges.push(Vec::new());
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        if !self.edges[a].iter().any(|x| *x==b) {
            self.edges[a].push(b)
        }
    }

    fn node_by_position(&self, x: usize, y: usize) -> usize {
        y * (self.width as usize) + x
    }

    fn node(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    fn nodes(&self) -> impl Iterator<Item=usize> {
        0..self.nodes.len()
    }

    fn height(&self) -> u32 {
        self.nodes.len() as u32/self.width
    }

    fn get_adjacent(&self, id: usize) -> &[usize] {
        &self.edges[id]
    }
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    {
        let mut g = Graph::new();
        for (i, line) in content.trim().split_whitespace().enumerate() {
            g.set_width(line.len() as u32);
            for (j, cost) in line.bytes().map(|x| (x - b'0') as u32).enumerate() {
                g.add_node(Node::new(j, i, cost));
            }
        }
        for id in g.nodes() {
            // Top
            if g.node(id).y != 0 {
                g.add_edge(id, g.node_by_position(g.node(id).x, g.node(id).y - 1));
                g.add_edge(g.node_by_position(g.node(id).x, g.node(id).y - 1), id);
            }
            // Left
            if g.node(id).x != 0 {
                g.add_edge(id, g.node_by_position(g.node(id).x - 1, g.node(id).y));
                g.add_edge(g.node_by_position(g.node(id).x - 1, g.node(id).y), id);
            }
            // Bottom
            if g.node(id).y != g.height() as usize - 1 {
                g.add_edge(id, g.node_by_position(g.node(id).x, g.node(id).y + 1));
                g.add_edge(g.node_by_position(g.node(id).x, g.node(id).y + 1), id);
            }
            // Right
            if g.node(id).x != g.width as usize - 1 {
                g.add_edge(id, g.node_by_position(g.node(id).x + 1, g.node(id).y));
                g.add_edge(g.node_by_position(g.node(id).x + 1, g.node(id).y), id);
            }
        }

        let start = g.node_by_position(0, 0);

        let mut routes = Vec::new();
        routes.resize_with(g.width as usize * g.height() as usize, || None);
        routes[0] = Some(0);
        djikstra(&g, start, &mut routes);

        println!("{}", routes[routes.len()-1].unwrap());
    }
    {
        let mut g = Graph::new();
        let height = content.trim().split_whitespace().count();
        let width = content.trim().split_whitespace().next().unwrap().len();
        for row_mult in 0..5 {
            for (i, line) in content.trim().split_whitespace().enumerate() {
                g.set_width(5*(line.len() as u32));
                for col_mult in 0..5 {
                    for (j, cost) in line.bytes().map(|x| (x - b'0') as u32).enumerate() {
                        let cost = cost + col_mult + row_mult;
                        let cost = if cost > 9 { 1 + (cost - 1) % 9 } else { cost };
                        g.add_node(Node::new(j+(col_mult as usize)*width, i+(row_mult as usize)*height, cost));
                    }
                }
            }
        }
        for id in g.nodes() {
            // Top
            if g.node(id).y != 0 {
                g.add_edge(id, g.node_by_position(g.node(id).x, g.node(id).y - 1));
                g.add_edge(g.node_by_position(g.node(id).x, g.node(id).y - 1), id);
            }
            // Left
            if g.node(id).x != 0 {
                g.add_edge(id, g.node_by_position(g.node(id).x - 1, g.node(id).y));
                g.add_edge(g.node_by_position(g.node(id).x - 1, g.node(id).y), id);
            }
            // Bottom
            if g.node(id).y != g.height() as usize - 1 {
                g.add_edge(id, g.node_by_position(g.node(id).x, g.node(id).y + 1));
                g.add_edge(g.node_by_position(g.node(id).x, g.node(id).y + 1), id);
            }
            // Right
            if g.node(id).x != g.width as usize - 1 {
                g.add_edge(id, g.node_by_position(g.node(id).x + 1, g.node(id).y));
                g.add_edge(g.node_by_position(g.node(id).x + 1, g.node(id).y), id);
            }
        }

        let start = g.node_by_position(0, 0);
        let mut routes = Vec::new();
        routes.resize_with(g.width as usize * g.height() as usize, || None);
        routes[0] = Some(0);

        djikstra(&g, start, &mut routes);
        println!("{}", routes[routes.len()-1].unwrap());
    }

}

fn djikstra(g: &Graph, start: usize, routes: &mut Vec<Option<u32>>) {
    let mut q = BinaryHeap::new();
    let mut visited = Vec::new();
    visited.resize_with(g.width as usize * g.height() as usize, || false);

    q.push((Reverse(g.node(start).cost), start));
    while let Some((_, next)) = q.pop() {
        visit(g, next, routes, &mut visited, &mut q);
        if routes[routes.len()-1].is_some() {
            break;
        }
    }
}

fn visit(g: &Graph, node: usize, routes: &mut Vec<Option<u32>>, visited: &mut Vec<bool>, q: &mut BinaryHeap<(Reverse<u32>, usize)>) {
    // We set the cost of every node before visiting it, so unwrap can't panic.
    let current_cost = routes[node].unwrap();

    visited[node] = true;

    for adj in g.get_adjacent(node) {
        let adj_cost = g.node(*adj).cost;
        if let Some(cost) = routes[*adj] {
            if current_cost + adj_cost < cost {
                routes[*adj] = Some(current_cost + adj_cost);
            }
        } else {
            routes[*adj] = Some(current_cost + adj_cost);
        }
        if !visited[*adj] && !q.iter().any(|x| x == &(Reverse(routes[*adj].unwrap()), *adj)) {
            q.push((Reverse(routes[*adj].unwrap()), *adj))
        }
    }
}

#[allow(dead_code)]
fn print_routes(routes: &[Option<u32>], width: u32) {
    for (i, r) in routes.iter().enumerate() {
        if let Some(r) = r {
            print!("{:3} ", r);
        } else {
            print!("... ");
        }
        if i as u32 % width == width-1 {
            println!();
        }
    }
    println!();
}

#[allow(dead_code)]
fn print_graph(g: &Graph) {
    for (i, r) in g.nodes.iter().enumerate() {
        print!("{}", r.cost);
        if i as u32 % g.width == g.width-1 {
            println!();
        }
    }
    println!();
}
