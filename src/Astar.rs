use nannou::prelude::*;
use crate::Nodes::Node;


pub struct AStar{
    pub start: Vec2,
    pub end: Vec2,
    pub walkers: Vec<Walker>,
}

pub struct Walker {
    pub position: Vec2,
    pub path: Vec<Node>,
}

impl Walker {
    pub fn new(start: &Node, path: Vec<Node>) -> Self{
        Self {
            position: start.position,
            path: path.clone(),
        }
    }
}

impl AStar {
    pub fn new(start: &Node, end: &Node) -> Self{
        AStar {
            start: start.position,
            end: end.position,
            walkers: vec![Walker::new(start, Vec::new())],
        }
    }

    pub fn step(&mut self){
        
    }
}