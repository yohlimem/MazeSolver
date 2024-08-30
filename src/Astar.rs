use std::rc::Rc;

use nannou::prelude::*;
use crate::Nodes::Node;


pub struct AStar{
    pub start: Vec2,
    pub end: Vec2,
    pub walkers: Vec<Walker>,
}

pub struct RandomStar{
    pub start: Vec2,
    pub end: Vec2,
    pub walkers: Vec<Walker>,
}

pub struct Walker {
    pub position: Vec2,
    pub path: Vec<Rc<Node>>,
}

impl Walker {
    pub fn new(start: Rc<Node>, path: Vec<Rc<Node>>) -> Self{
        Self {
            position: start.position,
            path: path.clone(),
        }
    }
}

impl MazeSolver for AStar {
    fn new(start: Rc<Node>, end: Rc<Node>) -> AStar{
        AStar {
            start: start.position,
            end: end.position,
            walkers: vec![Walker::new(Rc::clone(&start), Vec::new())],
        }
    }

    fn step(&mut self){
        
    }
}

impl MazeSolver for RandomStar {
    fn new(start: Rc<Node>, end: Rc<Node>) -> RandomStar{
        RandomStar {
            start: start.position,
            end: end.position,
            walkers: vec![Walker::new(start, Vec::new())],
        }
    }

    fn step(&mut self){
        // for walker in self.walkers {

        // }
    }
} 

pub trait MazeSolver {
    fn step(&mut self);
    fn new(start: Rc<Node>, end: Rc<Node>) -> Self;
    
}