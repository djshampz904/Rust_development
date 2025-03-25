use petgraph::graph::{
    NodeIndex, 
    UnGraph
};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}


impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}



fn main() {
    // check imports if they are there
    let mut graph = UnGraph::<&Fighter, f32>::new_undirected();

    let fighters = [
        Fighter::new("Ryu"),
        Fighter::new("Ken"),
        Fighter::new("Chun-Li"),
        Fighter::new("Guile"),
        Fighter::new("Zangief"),
        Fighter::new("Dhalsim"),
        Fighter::new("Blanka"),
        Fighter::new("E. Honda"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();


    add_edge(&mut graph, &fighter_nodes, 0, 1); // Ryu -> Ken
    add_edge(&mut graph, &fighter_nodes, 0, 2); // Ryu -> Chun-Li
    add_edge(&mut graph, &fighter_nodes, 1, 2); // Ken -> Chun-Li
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Ken -> Guile
    add_edge(&mut graph, &fighter_nodes, 2, 3); // Chun-Li -> Guile
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Chun-Li -> Zangief
    add_edge(&mut graph, &fighter_nodes, 3, 5); // Guile -> Dhalsim
    add_edge(&mut graph, &fighter_nodes, 4, 6); // Zangief -> Blanka
    add_edge(&mut graph, &fighter_nodes, 4, 7); // Zangief -> E. Honda
    add_edge(&mut graph, &fighter_nodes, 5, 6); // Dhalsim -> Blanka

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;

        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;

        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);
        
    }   

}