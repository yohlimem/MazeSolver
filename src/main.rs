use std::{time::Duration, vec};

use nannou::prelude::*;
mod Nodes;
mod Astar;
use crate::Astar::AStar;
use crate::Nodes::Node;
use std::collections::HashSet;

// TODO: Make it so it doesnt trace over the same node twice.
struct Model {
    _window: window::Id,
    nodes: Vec<Vec<Node>>,
    maze_size: usize,
    walker: (Vec2, Vec2, usize),
    explored_nodes: Vec<Vec2>,
    traced_nodes: HashSet<usize>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let mut nodes = Vec::new();


    let maze_size = 20;

    for i in 0..maze_size {
        let mut row = Vec::new();
        for j in 0..maze_size {
            row.push(Node::new(vec2((i as f32 - maze_size as f32 / 2.0) as f32, (j as f32 - maze_size as f32/2.0) as f32)));
        }
        nodes.push(row);
    }

    
    let time = std::time::Instant::now();
    
    generate_maze(maze_size, &mut nodes);
    let time2 = std::time::Instant::now();
    
    println!("Time to generate maze of size {maze_size} is {:?} ", time2 - time);
    println!("{:?}", nodes[0][0].connected_nodes);
    
    // println!("{:?}", );
    // nodes[1][0].8 {}",nodes[1][0].able_to_move_to(&nodes[9][0]));
    

    Model {
        _window,
        nodes,
        maze_size,
        walker: (vec2(0.0, 0.0), vec2(1.0, 0.0), 1),
        explored_nodes: vec![vec2(0.0, 0.0)],
        traced_nodes: HashSet::new(),
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    let direction_list = [vec2(1.0,0.0), vec2(0.0,1.0), vec2(-1.0,0.0), vec2(0.0,-1.0)];
    let random_dir = random_range(0, 4);
    let random_dir = direction_list[random_dir];
    
    if is_outside(model.walker.0 + random_dir, model.maze_size) {
        return;
    } 
    
    if already_explored(&model.explored_nodes, &(model.walker.0 + random_dir)){
        // walker.2 += 1;
        while model.traced_nodes.contains(&(model.explored_nodes.len() - model.walker.2)) && model.walker.2 < model.maze_size * model.maze_size {
            model.walker.2 += 1;
            // println!("walker.2: {}", walker.2);
        }
        let index = model.explored_nodes.len() - model.walker.2;
        model.walker.1 = model.walker.0;
        model.walker.0 = model.explored_nodes[index];

        // println!("index: {}", walker.2);
        let mut try_again = true;
        for dir in direction_list{
            if already_explored(&model.explored_nodes, &(model.walker.0 + dir)) {
                continue;
            }
            if is_outside(model.walker.0 + dir, model.maze_size) {
                continue;
            }
            model.walker.1 = model.walker.0;
            model.walker.0 += dir;
            // println!("new pos = {}, from_branch = {}, dir = {dir}", walker.0, walker.0 - dir);
            try_again = false;
            break;
            
        }
        if try_again {

            // explored_nodes.remove(explored_nodes.len() - walker.2);
            // println!("try again");
            model.traced_nodes.insert(model.explored_nodes.len() - model.walker.2);
            return;
        }
    } else {

        // println!("normal");
        model.walker.1 = model.walker.0; // set last pose to this
        model.walker.0 += random_dir; // set new pose
    }

    
    model.walker.2 = 1; // set tracer index to 1

    model.explored_nodes.push(model.walker.0);
    // println!("{}, {}", walker.0, walker.1);
    Node::connect((model.walker.0.x as usize, model.walker.0.y as usize), (model.walker.1.x as usize, model.walker.1.y as usize), &mut model.nodes);
    

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Rgb8::new(30, 203, 225));

    for row in &model.nodes {
        for node in row {
            node.draw(&draw, GREY);
            if node.is_connected() {
                node.draw_connection(&draw)

            }
        }
    }
    model.nodes[0][0].draw(&draw, RED);

    // model.walker

    

    draw.to_frame(app, &frame).unwrap();
}

fn is_outside(next_pos: Vec2, maze_size: usize) -> bool{
    next_pos.x > maze_size as f32 - 1.0 || next_pos.y > maze_size as f32 - 1.0 || next_pos.y < 0.0 || next_pos.x < 0.0
}

fn already_explored(explored_nodes_list: &Vec<Vec2>, pos: &Vec2) -> bool{
    explored_nodes_list.contains(pos)
}

fn generate_maze(maze_size: usize, nodes: &mut Vec<Vec<Node>>){
        // if model.maze_size * model.maze_size <= model.walker.2 {return} // dont crash
    
    

}
