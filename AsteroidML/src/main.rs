use rand::{Rng, thread_rng};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

mod grid_world;
use grid_world::*;

#[derive(Debug)]
struct Spaceship{
    x: u32,
    y: u32,
}

fn main() {
    let mut grid_world = GridWorld::new(6, 6);
    init_grid_world(&mut grid_world);
    let policy = get_grid_world_policy(&grid_world);
    
    let mut grid_values = HashMap::new();
    grid_values.insert(WorldObjectTypes::Asteroid, None);
    grid_values.insert(WorldObjectTypes::Empty, Some(-1));
    grid_values.insert(WorldObjectTypes::Goal, Some(10));



    println!("{}", grid_world);
}

fn init_grid_world(grid_world: &mut GridWorld){
    grid_world.set_grid(0, 0, WorldObjectTypes::Asteroid);
    grid_world.set_grid(1, 0, WorldObjectTypes::Asteroid);
    grid_world.set_grid(3, 0, WorldObjectTypes::Asteroid);
    
    grid_world.set_grid(2, 1, WorldObjectTypes::Asteroid);
    
    grid_world.set_grid(4, 2, WorldObjectTypes::Asteroid);
    grid_world.set_grid(5, 2, WorldObjectTypes::Asteroid);
    
    grid_world.set_grid(2, 3, WorldObjectTypes::Asteroid);
    grid_world.set_grid(5, 3, WorldObjectTypes::Asteroid);

    grid_world.set_grid(0, 4, WorldObjectTypes::Asteroid);
    grid_world.set_grid(3, 4, WorldObjectTypes::Asteroid);

    grid_world.set_grid(5, 0, WorldObjectTypes::Goal);
}

fn state_transition_probability_function(grid_world: &GridWorld, action: Moves, curr_position: (u32, u32), next_position: (u32, u32)) -> f64{
    if curr_position == next_position{
        return match curr_position{
            //possibility to stay in current position with taking the correct way
            (x, y) if (x == 0 && (action == Moves::Left || action == Moves::LeftJump)) 
                || (x == grid_world.x_length - 1 && (action == Moves::Right || action == Moves::RightJump)) 
                || (y == 0 && (action == Moves::Left || action == Moves::LeftJump))
                || (y == grid_world.y_length - 1 && (action == Moves::Down || action == Moves::DownJump))
            => 0.9,
            (x, y) if (x == 0 || x == grid_world.x_length - 1) && (action == Moves::Up || action == Moves::UpJump || action == Moves::Down || action == Moves::DownJump)
                ||  (y == 0 || y == grid_world.y_length - 1) && (action == Moves::Left || action == Moves::LeftJump || action == Moves::Right || action == Moves::RightJump)
            => 0.05,
            _ => 0.0,
        };
    }else{

    }
    
    0.0
}

fn get_grid_world_value(grid_world: &GridWorld) -> Vec<f64>{
    let mut value = Vec::new();

    for i in 0..(grid_world.y_length * grid_world.y_length){
        
    }
    value
}

fn get_grid_world_policy(grid_world: &GridWorld) -> Vec<Moves>{
    let mut value = Vec::new();
    let mut rng = rand::thread_rng();
    

    for _ in 0..(grid_world.y_length * grid_world.y_length){
        value.push(
            match rng.gen_range(0..7){
                0 => Moves::Up,
                1 => Moves::Right,
                2 => Moves::Down,
                3 => Moves::Left,
                4 => Moves::UpJump,
                5 => Moves::RightJump,
                6 => Moves::DownJump,
                7 => Moves::LeftJump,
                _ => Moves::Up,
            }
        );
    }
    value
}