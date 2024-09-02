use std::{borrow::Borrow, cell::{Ref, RefCell}, rc::Rc};

use nannou::{draw::{mesh::vertex::Color, properties::spatial::position}, prelude::*};

#[derive(PartialEq, Debug, Clone)]
pub enum Connection {
    In(Rc<Node>),
    Out(Rc<Node>),
}

impl Connection {
    pub fn get_position(&self) -> Vec2 {
        match self {
            Connection::In(node) => node.position,
            Connection::Out(node) => node.position,
        }
    }

    pub fn is_out (&self) -> bool {
        match self {
            Connection::Out(_) => true,
            _ => false,
        }
    }

    pub fn all_out(connections: Ref<Vec<Connection>>) -> Vec<Connection> {
        connections.clone().into_iter().filter(|node| node.is_out()).collect()
    }

    pub fn get_out(&self) -> Rc<Node> {
        match self {
            Connection::Out(node) => Rc::clone(node),
            _ => panic!("Can't get out of in connection"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Node {
    pub position: Vec2,
    pub connected_nodes: RefCell<Vec<Connection>>,
} 

impl Node {
    pub const DIST:f32 = 20.0;
    pub const RAD:f32 = 10.0;
    pub fn new(position: Vec2) -> Self {
        Node {
            position,
            connected_nodes: RefCell::new(Vec::new()),
        }
    }
    fn get_neighbors_positions(node: &Node) -> Vec<Vec2> {
        let directions = vec![
            vec2(0.0, 1.0),  // Up
            vec2(0.0, -1.0), // Down
            vec2(1.0, 0.0),  // Right
            vec2(-1.0, 0.0), // Left
        ];
        
        directions.into_iter().map(|dir| node.position + dir).collect()
    }
    pub fn contains(&self, other: &Node) -> bool {
        if self.connected_nodes.borrow().len() == 0 {
            return false;
        }
        self.connected_nodes.borrow().iter().any(|conn| {
            match conn {
                Connection::Out(node) | Connection::In(node) => node.position == other.position,
            }
        })
    }
    pub fn draw(&self, draw: &Draw, color: Srgba<u8>) {
        
        let wall_length = Node::DIST; // The length of each wall

        // Draw the node itself (optional, e.g., as a small circle)
        draw.ellipse()
            .x_y(self.position.x * Node::DIST, self.position.y * Node::DIST)
            .radius(Node::RAD)
            .color(color);
        
        // Get positions of potential neighbors
        let neighbors_positions = Self::get_neighbors_positions(self);

        // Check which neighbors are connected and draw walls where they are not connected
        for (i, neighbor_pos) in neighbors_positions.iter().enumerate() {
            let is_connected = self.connected_nodes.borrow().iter().any(|conn| {
                match conn {
                    Connection::Out(node) | Connection::In(node) => node.position == *neighbor_pos,
                }
            });

            if !is_connected {
                // Draw the wall
                match i {
                    0 => { // Up
                        draw.line()
                            .start(self.position * Node::DIST + vec2(-wall_length / 2.0, wall_length / 2.0))
                            .end(self.position * Node::DIST + vec2(wall_length / 2.0, wall_length / 2.0))
                            .color(BLACK);
                    },
                    1 => { // Down
                        draw.line()
                            .start(self.position * Node::DIST + vec2(-wall_length / 2.0, -wall_length / 2.0))
                            .end(self.position * Node::DIST + vec2(wall_length / 2.0, -wall_length / 2.0))
                            .color(BLACK);
                    },
                    2 => { // Right
                        draw.line()
                            .start(self.position * Node::DIST + vec2(wall_length / 2.0, -wall_length / 2.0))
                            .end(self.position * Node::DIST + vec2(wall_length / 2.0, wall_length / 2.0))
                            .color(BLACK);
                    },
                    3 => { // Left
                        draw.line()
                            .start(self.position * Node::DIST + vec2(-wall_length / 2.0, -wall_length / 2.0))
                            .end(self.position * Node::DIST + vec2(-wall_length / 2.0, wall_length / 2.0))
                            .color(BLACK);
                    },
                    _ => (),
                }
            }
        }
    }

    pub fn connect(self_index: (usize, usize), other_index: (usize, usize), nodes: &mut Vec<Vec<Rc<Node>>>) {
        assert!(nodes[self_index.0][self_index.1].position != nodes[other_index.0][other_index.1].position, "Can't connect a node to itself");
        assert!(!nodes[self_index.0][self_index.1].contains(nodes[other_index.0][other_index.1].borrow()), "Can't connect a node twice {:?}, {}", nodes[self_index.0][self_index.1].connected_nodes, nodes[other_index.0][other_index.1]);
        
        
        
        
        // we connect a node
        // out going for us, in going for them
        let node = Rc::clone(&nodes[other_index.0][other_index.1]);
        // let last_nodes = &mut nodes[self_index.0][self_index.1].connected_nodes;
        nodes[self_index.0][self_index.1].connected_nodes.borrow_mut().push(Connection::In(Rc::clone(&node)));
        
        // println!("connecting: {:?} to {:?}", self_index, other_index);
        
        

        nodes[other_index.0][other_index.1].connected_nodes.borrow_mut().push(Connection::Out(Rc::clone(&nodes[self_index.0][self_index.1])));
        // println!("node: {:?} out is {}", nodes[other_index.0][other_index.1])

    }

    pub fn draw_connection(&self, draw: &Draw) {
        // if self.position == vec2(-5.0, -5.0) {
        //     println!("drawing connection for {:?}", self.connected_nodes);
        // }
        self.connected_nodes.borrow().iter().for_each(|node| {
            match node {
                Connection::In(vec) => draw.arrow()
                    .start(vec.position * Self::DIST)
                    .end(self.position * Self::DIST)
                    .color(BLACK),
                Connection::Out(vec) => draw.arrow()
                        .start(self.position * Self::DIST)
                        .end(vec.position * Self::DIST)
                        .color(BLACK),
            };
        });
        
    }

    pub fn print_nodes(&self) {
        let position = self.position;

        let connected_nodes = self.connected_nodes.clone();
        println!("position: {}, connected nodes: {:?}", position, connected_nodes);
    }

    pub fn is_connected(&self) -> bool{
        self.connected_nodes.borrow().len() > 0
    }
}

impl Node {
    pub fn able_to_move_to(&self, to: &Node) -> bool {
        self.connected_nodes.borrow().iter().any(|node| {
            match node {
                Connection::Out(vec) => vec.position == to.position,
                _ => false,
            }
        })

    }
    pub fn get_random_neighbour(&self) -> Rc<Node> {
        let index = random_range(0, self.connected_nodes.borrow().len());
        match &self.connected_nodes.borrow()[index] {
            Connection::Out(node) => Rc::clone(node),
            _ => panic!("Can't get random neighbour"),
        }
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