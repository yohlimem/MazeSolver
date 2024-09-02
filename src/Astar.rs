use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use nannou::{lyon::algorithms::walk, prelude::*};
use crate::Nodes::{Connection, Node};
use std::collections::HashSet;

#[derive(PartialEq)]
pub enum Done {
    Found,
    NotFound(bool), // stuck, or not finished
}

pub struct AStar{
    pub start: Rc<Node>,
    pub end: Rc<Node>,
    pub path: Vec<Rc<Node>>,
    pub walkers: Vec<Walker>,
}

pub struct RandomStar{
    pub start: Rc<Node>,
    pub end: Rc<Node>,
    pub path: Vec<Rc<Node>>,
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
            is_done: Done::NotFound(false),
            path,
        }
    }

    pub fn step(&mut self, next_node: Rc<Node> ){
        self.current_node = RefCell::new(Rc::clone(&next_node));
        self.path.push(Rc::clone(&next_node))
    }

}

impl MazeSolver for AStar {
    fn new(start: Rc<Node>, end: Rc<Node>) -> AStar{
        AStar {
            start: Rc::clone(&start),
            end: Rc::clone(&end),
            walkers: vec![Walker::new(start, Vec::new())],
            path: Vec::new(),
        }
    }

    fn step(&mut self){
        let mut new_walkers: Vec<Walker> = Vec::new();
        for walker in &mut self.walkers {
            if let Done::Found = walker.is_done {
                // continue;
            }
            if let Done::NotFound(x) = walker.is_done {
                if x {
                    continue;
                }
            }
            // walker should walk until it reaches a split
            // it should keep walking but now clone itself for the split
            let next_nodes: Vec<Connection>;
            {
                let current_node = walker.current_node.borrow();
                next_nodes = Connection::all_out(current_node.connected_nodes.borrow());
            }
            if next_nodes.len() > 1 {
                    for i in 1..next_nodes.len() {
                        if self.path.contains(next_nodes[i].get_out().borrow()) {
                            println!("stopped");
                            walker.is_done = Done::NotFound(true);
                            continue;
                        }
                        // println!("new_walker");
                        new_walkers.push(Walker::new(Rc::clone(&walker.current_node.borrow()), walker.path.clone()));
                        self.path.push(Rc::clone(next_nodes[i].get_out().borrow()));
                        new_walkers.last_mut().unwrap().step(next_nodes[i].get_out());
                    }
            }
            if walker.current_node.borrow().position == self.end.position {
                println!("found end");
                walker.is_done = Done::Found;
                continue;
            }
            if next_nodes.len() == 0 {
                // println!("next_nodes: {:?}, current_node: {:?}", next_nodes, walker.current_node.borrow());
                // println!("No More");
                walker.is_done = Done::NotFound(true);
                continue;
            }
            
            walker.step(next_nodes[0].get_out());
            self.path.push(Rc::clone(&walker.current_node.borrow()));
        }
        self.walkers.append(&mut new_walkers);
    }
}

impl MazeSolver for RandomStar {
    fn new(start: Rc<Node>, end: Rc<Node>) -> RandomStar{
        RandomStar {
            start: Rc::clone(&start),
            end: Rc::clone(&end),
            walkers: vec![Walker::new(start, Vec::new())],
            path: Vec::new(),

        }
    }

    fn step(&mut self){
        let mut new_walkers: Vec<Walker> = Vec::new();
        for walker in &mut self.walkers {
            if let Done::Found = walker.is_done {
                // continue;
            }
            if let Done::NotFound(x) = walker.is_done {
                if x {
                    continue;
                }
            }
            // walker should walk until it reaches a split
            // it should keep walking but now clone itself for the split
            let next_nodes: Vec<Connection>;
            {
                let current_node = walker.current_node.borrow();
                next_nodes = Connection::all_out(current_node.connected_nodes.borrow());
            }
            if next_nodes.len() > 1 {
                    for i in 1..next_nodes.len() {
                        if self.path.contains(next_nodes[i].get_out().borrow()) {
                            println!("stopped");
                            walker.is_done = Done::NotFound(true);
                            continue;
                        }
                        // println!("new_walker");
                        new_walkers.push(Walker::new(Rc::clone(&walker.current_node.borrow()), walker.path.clone()));
                        self.path.push(Rc::clone(next_nodes[i].get_out().borrow()));
                        new_walkers.last_mut().unwrap().step(next_nodes[i].get_out());
                    }
            }
            if walker.current_node.borrow().position == self.end.position {
                println!("found end");
                walker.is_done = Done::Found;
                continue;
            }
            if next_nodes.len() == 0 {
                // println!("next_nodes: {:?}, current_node: {:?}", next_nodes, walker.current_node.borrow());
                // println!("No More");
                walker.is_done = Done::NotFound(true);
                continue;
            }
            
            walker.step(next_nodes[0].get_out());
            self.path.push(Rc::clone(&walker.current_node.borrow()));
        }
        self.walkers.append(&mut new_walkers);
    }
} 

pub trait MazeSolver {
    fn step(&mut self);
    fn new(start: Rc<Node>, end: Rc<Node>) -> Self;
    
}