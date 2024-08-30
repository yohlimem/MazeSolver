use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use nannou::{lyon::algorithms::walk, prelude::*};
use crate::Nodes::{Connection, Node};

pub enum Done {
    Found,
    NotFound(bool), // stuck, or not finished
}

pub struct AStar{
    pub start: Rc<Node>,
    pub end: Rc<Node>,
    pub walkers: Vec<Walker>,
}

pub struct RandomStar{
    pub start: Rc<Node>,
    pub end: Rc<Node>,
    pub walkers: Vec<Walker>,
}

pub struct Walker {
    pub current_node: RefCell<Rc<Node>>,
    pub path: Vec<Rc<Node>>,
    pub is_done: Done,
}

impl Walker {
    pub fn new(start: Rc<Node>, path: Vec<Rc<Node>>) -> Self{
        Self {
            current_node: RefCell::new(Rc::clone(&start)),
            path: path.clone(),
            is_done: Done::NotFound(false),
        }
    }

    pub fn step(&mut self, next_node: Rc<Node> ){
        self.current_node = RefCell::new(Rc::clone(&next_node));
        self.path.push(Rc::clone(&next_node));
    }

}

impl MazeSolver for AStar {
    fn new(start: Rc<Node>, end: Rc<Node>) -> AStar{
        AStar {
            start: Rc::clone(&start),
            end: Rc::clone(&end),
            walkers: vec![Walker::new(Rc::clone(&start), Vec::new())],
        }
    }

    fn step(&mut self){
        
    }
}

impl MazeSolver for RandomStar {
    fn new(start: Rc<Node>, end: Rc<Node>) -> RandomStar{
        RandomStar {
            start: Rc::clone(&start),
            end: Rc::clone(&end),
            walkers: vec![Walker::new(start, Vec::new())],
        }
    }

    fn step(&mut self){
        let mut new_walkers: Vec<Walker> = Vec::new();
        for walker in &mut self.walkers {
            // walker should walk until it reaches a split
            // it should keep walking but now clone itself for the split
            let next_nodes: Vec<Connection>;
            {
                let current_node = walker.current_node.borrow();
                next_nodes = Connection::all_out(current_node.connected_nodes.borrow());
            }
            if next_nodes.len() > 1 {
                    for i in 1..next_nodes.len() {
                        new_walkers.push(Walker::new(Rc::clone(&walker.current_node.borrow()), walker.path.clone()));
                        new_walkers.last_mut().unwrap().step(next_nodes[i].get_out());
                    }
            }
            if next_nodes.len() == 0 {
                // println!("next_nodes: {:?}, current_node: {:?}", next_nodes, walker.current_node.borrow());
                walker.is_done = Done::NotFound(true);
                continue;
            }
            walker.step(next_nodes[0].get_out());
            if walker.current_node.borrow().position == self.end.position {
                println!("found end");
                walker.is_done = Done::Found;
                return;
            }
        }
        self.walkers.append(&mut new_walkers);
    }
} 

pub trait MazeSolver {
    fn step(&mut self);
    fn new(start: Rc<Node>, end: Rc<Node>) -> Self;
    
}