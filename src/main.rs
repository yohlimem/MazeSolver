use std::borrow::Borrow;
use std::rc::Rc;
use std::{time::Duration, vec};

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

mod Nodes;
mod Astar;
use crate::Astar::AStar;
use crate::Astar::RandomStar;
use Astar::{Done, MazeSolver};
use crate::Nodes::Node;
use std::collections::HashSet;

// TODO: Make it so it doesnt trace over the same node twice.
struct Model {
    nodes: Vec<Vec<Rc<Node>>>,
    maze_size: usize,
    stop: bool,
    step_button: bool,
    random_star: RandomStar,
    egui: Egui,

}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    let mut nodes = Vec::new();


    let maze_size = 30;

    for i in 0..maze_size {
        let mut row = Vec::new();
        for j in 0..maze_size {
            row.push(Rc::new(Node::new(vec2((i as f32 - maze_size as f32 / 2.0) as f32, (j as f32 - maze_size as f32/2.0) as f32))));
        }
        nodes.push(row);
    }

    
    // let time = std::time::Instant::now();
    
    generate_maze(maze_size, &mut nodes);
    // let time2 = std::time::Instant::now();
    
    // println!("Time to generate maze of size {maze_size} is {:?} ", time2 - time);
    
    // println!("{:?}, {:?}", nodes[0][0], nodes[0][0].connected_nodes);
    // nodes[1][0].8 {}",nodes[1][0].able_to_move_to(&nodes[9][0]));

    // println!("first node: {:?}", nodes[0][1].connected_nodes);

    // connect random nodes that are beside each other
    // for i in 0..50 {

    //     let direction_list = [vec2(1.0,0.0), vec2(0.0,1.0), vec2(-1.0,0.0), vec2(0.0,-1.0)];
    //     let dir = direction_list[random_range(0, 4)];
    
    //     // choose random node to connect
    //     let random_node = vec2(random_range(0, maze_size) as  f32, random_range(0, maze_size) as f32);
    //     let next_node = vec2(random_node.x + dir.x, random_node.y + dir.y);

    //     if is_outside(random_node, maze_size) || is_outside(next_node, maze_size) {
    //         continue;
    //     }


    //     if nodes[random_node.x as usize][random_node.y as usize].contains(nodes[next_node.x as usize][next_node.y as usize].borrow()) {
    //         continue;       
    //     }
    //     if nodes[next_node.x as usize][next_node.y as usize].contains(nodes[random_node.x as usize][random_node.y as usize].borrow()) {
    //         continue;
    //     }

    //     let random_node = (random_node.x as usize, random_node.y as usize);
    //     let next_node = (next_node.x as usize, next_node.y as usize);

    //     // convert to tuple of usize
    
    //     Node::connect(random_node, next_node, &mut nodes);
    // }


    let a_random = RandomStar::new(Rc::clone(&nodes[0][0]), Rc::clone(&nodes[0][maze_size - 1]));
    

    Model {
        nodes,
        maze_size,
        random_star: a_random,
        egui,
        stop: false,
        step_button: false,
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    render_egui(&mut model.egui, &mut model.nodes, &mut model.random_star, model.maze_size, &mut model.stop, &mut model.step_button);

    // let mouse_pos = app.mouse.position();
    // for row in &model.nodes {
    //     for node in row {
    //         if (mouse_pos - node.position*Node::DIST).length() <= Node::RAD {
    //             println!("pos: {}, connections: {:?}", node.position, node.connected_nodes);
    //         }
    //     }
    // }
    // if app.elapsed_frames() % 10 == 0 {
    //     model.random_star.step();
    // }
    if model.step_button {
        model.random_star.step();
        
    }
    if model.stop {
        return;
    }
    model.random_star.step();

    

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Rgb8::new(30, 203, 225));

    for row in &model.nodes {
        for node in row {
            node.draw(&draw, srgba8(0,0,0,0));
            // if node.is_connected() {
            //     node.draw_connection(&draw)

            // }
        }
    }
    model.nodes[0][0].draw(&draw, RED.into());

    for node in 1..model.random_star.path.len() {
        model.random_star.path[node-1].draw(&draw, GREEN.into());
    }
    for walker in model.random_star.walkers.iter() {
        match walker.is_done {
            Done::Found => {
            for node in 1..walker.path.len() {

                walker.current_node.borrow().draw(&draw, GREEN.into());
                // draw arrows for connections
                draw.arrow()
                    .start(walker.path[node-1].position*Node::DIST)
                    .end(walker.path[node].position*Node::DIST)
                    .weight(2.0)
                    .color(BLUE);
                }
                    
            },
            Done::NotFound(x) => {
                if x {
                    walker.current_node.borrow().draw(&draw, BLUE.into());
                } else {
                    walker.current_node.borrow().draw(&draw, RED.into());

                }
                continue;
            },
        }
    }


    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();

}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}
fn render_egui(egui: &mut Egui, nodes: &mut Vec<Vec<Rc<Node>>>, random_star: &mut RandomStar, maze_size: usize, stop: &mut bool, step: &mut bool){
    let egui = egui;
    // egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        // ui.label("res"); // template
        // ui.add(egui::Slider::new(&mut model.num, 1.0..=40.0));
        ui.label("reset");
        let reset_button = ui.button("reset!").clicked();
        if reset_button {
            reset(nodes, random_star, maze_size);
        }
        let stop_button = ui.button("stop").clicked();
        if stop_button {
            *stop = !*stop;
        }
        *step = ui.button("step").clicked();
    });
}
fn is_outside(next_pos: Vec2, maze_size: usize) -> bool{
    next_pos.x > maze_size as f32 - 1.0 || next_pos.y > maze_size as f32 - 1.0 || next_pos.y < 0.0 || next_pos.x < 0.0
}

fn already_explored(explored_nodes_list: &Vec<Vec2>, pos: &Vec2) -> bool{
    explored_nodes_list.contains(pos)
}

fn generate_maze(maze_size: usize, nodes: &mut Vec<Vec<Rc<Node>>>){
        // if model.maze_size * model.maze_size <= model.walker.2 {return} // dont crash
    let mut walker: (Vec2, Vec2, usize) = (vec2(0.0, 0.0), vec2(1.0, 0.0), 1);
    let mut explored_nodes = vec![vec2(0.0, 0.0)];
    let mut traced_nodes = HashSet::new();

    let direction_list = [vec2(1.0,0.0), vec2(0.0,1.0), vec2(-1.0,0.0), vec2(0.0,-1.0)];


    while explored_nodes.len() <= maze_size * maze_size && walker.2 < maze_size * maze_size {
        // println!("explored_nodes.len(): {} , maze_size: {}", explored_nodes.len(), maze_size * maze_size);

        let random_dir = random_range(0, 4);
        let random_dir = direction_list[random_dir];
        
        if is_outside(walker.0 + random_dir, maze_size) {
            continue
        } 
        
        if already_explored(&explored_nodes, &(walker.0 + random_dir)){
            // walker.2 += 1;
            while traced_nodes.contains(&(explored_nodes.len() - walker.2)) && walker.2 < maze_size * maze_size {
                walker.2 += 1;
                // println!("walker.2: {}", walker.2);
            }
            let index = explored_nodes.len() - walker.2;
            walker.1 = walker.0;
            walker.0 = explored_nodes[index];
    
            // println!("index: {}", walker.2);
            let mut try_again = true;
            for dir in direction_list{
                if already_explored(&explored_nodes, &(walker.0 + dir)) {
                    continue;
                }
                if is_outside(walker.0 + dir, maze_size) {
                    continue;
                }
                walker.1 = walker.0;
                walker.0 += dir;
                // println!("new pos = {}, from_branch = {}, dir = {dir}", walker.0, walker.0 - dir);
                try_again = false;
                break;
                
            }
            if try_again {
    
                // explored_nodes.remove(explored_nodes.len() - walker.2);
                // println!("try again");
                traced_nodes.insert(explored_nodes.len() - walker.2);
                continue;
            }
        } else {
    
            // println!("normal");
            walker.1 = walker.0; // set last pose to this
            walker.0 += random_dir; // set new pose
        }
    
        
        walker.2 = 1; // set tracer index to 1
    
        explored_nodes.push(walker.0);
        // println!("{}, {}", walker.0, walker.1);
        Node::connect((walker.0.x as usize, walker.0.y as usize), (walker.1.x as usize, walker.1.y as usize), nodes);
    }
    

}

fn reset(nodes: &mut Vec<Vec<Rc<Node>>>, random_star: &mut RandomStar, maze_size: usize){
    nodes.iter().for_each(|row| {
        row.iter().for_each(|node| {
            node.connected_nodes.borrow_mut().clear();
        })
    });
    *random_star = RandomStar::new(Rc::clone(&nodes[0][0]), Rc::clone(&nodes[maze_size - 1][maze_size - 1]));
    generate_maze(maze_size, nodes);

}