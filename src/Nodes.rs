use std::{cell::{Ref, RefCell}, rc::Rc};

use nannou::{draw::{mesh::vertex::Color, properties::spatial::position}, prelude::*};

#[derive(PartialEq, Debug, Clone)]
pub enum Connection {
    In(Rc<Node>),
    Out(Rc<Node>),
}

#[derive(PartialEq, Clone)]
pub struct Node {
    pub position: Vec2,
    pub connected_nodes: RefCell<Vec<Connection>>,
} 

impl Node {
    pub const DIST:f32 = 50.0;
    pub const RAD:f32 = 10.0;
    pub fn new(position: Vec2) -> Self {
        Node {
            position,
            connected_nodes: RefCell::new(Vec::new()),
        }
    }

    pub fn draw(&self, draw: &Draw, color: Srgb<u8> ) {
        draw.ellipse()
            .xy(self.position*Self::DIST)
            .radius(Self::RAD)
            .color(color);
    }

    pub fn connect(self_index: (usize, usize), other_index: (usize, usize), nodes: &mut Vec<Vec<Rc<Node>>>) {

        assert!(nodes[self_index.0][self_index.1].position != nodes[other_index.0][other_index.1].position, "Can't connect a node to itself");



        
        // we connect a node
        // out going for us, in going for them
        let node = Rc::clone(&nodes[other_index.0][other_index.1]);
        // let last_nodes = &mut nodes[self_index.0][self_index.1].connected_nodes;
        nodes[self_index.0][self_index.1].connected_nodes.borrow_mut().push(Connection::In(Rc::clone(&node)));
            
        
        

        nodes[other_index.0][other_index.1].connected_nodes.borrow_mut().push(Connection::Out(Rc::clone(&nodes[other_index.0][other_index.1])));

    }

    pub fn draw_connection(&self, draw: &Draw) {
        &self.connected_nodes.borrow().iter().for_each(|node| {
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