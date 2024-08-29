use nannou::{draw::{mesh::vertex::Color, properties::spatial::position}, prelude::*};

#[derive(PartialEq, Debug, Clone)]
pub enum Connection {
    In(Vec2),
    Out(Vec2),
}

#[derive(PartialEq, Clone)]
pub struct Node {
    pub position: Vec2,
    pub connected_nodes: Option<Vec<Connection>>,
} 

impl Node {
    pub const DIST:f32 = 50.0;
    pub const RAD:f32 = 10.0;
    pub fn new(position: Vec2) -> Self {
        Node {
            position,
            connected_nodes: None,
        }
    }

    pub fn draw(&self, draw: &Draw, color: Srgb<u8> ) {
        draw.ellipse()
            .xy(self.position*Self::DIST)
            .radius(Self::RAD)
            .color(color);
    }

    pub fn connect(self_index: (usize, usize), other_index: (usize, usize), nodes: &mut Vec<Vec<Node>>) {

        assert!(nodes[self_index.0][self_index.1].position != nodes[other_index.0][other_index.1].position, "Can't connect a node to itself");



        
        // we connect a node
        // out going for us, in going for them
        if nodes[self_index.0][self_index.1].connected_nodes.is_none() {
            nodes[self_index.0][self_index.1].connected_nodes = Some(vec![Connection::Out(nodes[other_index.0][other_index.1].position)]);
        } else {
            let position = (nodes[other_index.0][other_index.1].position).clone();
            if let Some(last_nodes) = &mut nodes[self_index.0][self_index.1].connected_nodes {
                last_nodes.push(Connection::Out(position));
            }
        }
        
        if nodes[other_index.0][other_index.1].connected_nodes.is_none() {
            nodes[other_index.0][other_index.1].connected_nodes = Some(vec![Connection::In(nodes[self_index.0][self_index.1].position)]);
        } else {
            let position = (nodes[self_index.0][self_index.1].position).clone();
            if let Some(last_nodes) = &mut nodes[other_index.0][other_index.1].connected_nodes {
                last_nodes.push(Connection::In(position));
            }
        }
        // if let Some((last_nodes)) = &nodes[self_index.0][self_index.1].connected_nodes {
        //     if last_nodes.contains(&Connection::Out(nodes[other_index.0][other_index.1].position)) {
        //         return;
        //     }
        //     if last_nodes.contains(&Connection::In(nodes[other_index.0][other_index.1].position)) {
        //         return;
        //     }   
        // }
        
        // if let Some(last_nodes) = &nodes[other_index.0][other_index.1].connected_nodes {
        //     if last_nodes.contains(&Connection::Out(nodes[self_index.0][self_index.1].position)) {
        //         return;
        //     }
        //     if last_nodes.contains(&Connection::In(nodes[self_index.0][self_index.1].position)) {
        //         return;
        //     }
        // }

        

    }

    pub fn draw_connection(&self, draw: &Draw) {
        if let Some(last_node) = &self.connected_nodes {
            last_node.iter().for_each(|node| {
                match node {
                    Connection::In(vec) => draw.arrow()
                        .start(*vec * Self::DIST)
                        .end(self.position * Self::DIST)
                        .color(BLACK),
                    Connection::Out(vec) => draw.arrow()
                            .start(self.position * Self::DIST)
                            .end(*vec * Self::DIST)
                            .color(BLACK),
                };
            });
        }
    }

    pub fn print_nodes(&self) {
        let position = self.position;

        let connected_nodes = self.connected_nodes.clone().unwrap();
        println!("position: {}, connected nodes: {:?}", position, connected_nodes);
    }

    pub fn is_connected(&self) -> bool{
        self.connected_nodes.is_some()
    }
}

impl Node {
    pub fn able_to_move_to(&self, to: &Node) -> bool {
        self.connected_nodes.clone().unwrap().iter().any(|node| {
            match node {
                Connection::Out(vec) => vec == &to.position,
                _ => false,
            }
        })

    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position)
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position)
        
    }
}