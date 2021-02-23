use std::fmt;
use rand::{Rng, thread_rng};
use std::collections::HashMap;


#[derive(Debug)]
enum GameObjectTypes{
    Asteroid,
    Empty,
    Goal
}

#[derive(Debug)]
enum Moves{
    Up,
    Down, 
    Left, 
    Right
}

#[derive(Debug)]
struct GridWorld{
    x_length: u32,
    y_length: u32,
    grids: Vec<Grid>,
}
impl GridWorld{
    fn new(x_size: u32, y_size: u32) -> Self{
        let mut tmp_grid = Vec::new();

        for y in 0..y_size{
            for x in 0..x_size{
                tmp_grid.push(Grid{x: x, y: y, content: GameObjectTypes::Empty});
            }
        }
        GridWorld{x_length: x_size, y_length: y_size, grids: tmp_grid}
    }
    fn get_grid(&mut self, x: usize, y: usize) -> Result<&mut Grid, String>{
        if x >= self.x_length as usize || y >= self.y_length as usize{
            return Err(String::from("dimensions do not fit"));
        }

        Ok(&mut self.grids[(x + y * self.x_length as usize) as usize])
    }
}
impl fmt::Display for GridWorld{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        for y in 0..self.y_length{
            for x in 0..self.x_length{
                write!(f, "{:?}\t", self.grids[(x + y*self.x_length) as usize].content);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Grid{
    x: u32,
    y: u32,
    content: GameObjectTypes,
}

#[derive(Debug)]
struct Spaceship{
    x: u32,
    y: u32,
}

fn main() {
    let mut grid_world = GridWorld::new(6, 6);

    init_grid_world(&mut grid_world);
    let ss = Spaceship{x:0, y:0};

    println!("{:?}", get_grid_world_policy(&grid_world));
    println!("{:?}", get_grid_world_value(&grid_world));
    


}

fn init_grid_world(grid_world: &mut GridWorld){
    grid_world.get_grid(0, 0).unwrap().content = GameObjectTypes::Asteroid;
    grid_world.get_grid(1, 0).unwrap().content = GameObjectTypes::Asteroid;
    grid_world.get_grid(3, 0).unwrap().content = GameObjectTypes::Asteroid;
    
    grid_world.get_grid(2, 1).unwrap().content = GameObjectTypes::Asteroid;
    
    grid_world.get_grid(4, 2).unwrap().content = GameObjectTypes::Asteroid;
    grid_world.get_grid(5, 2).unwrap().content = GameObjectTypes::Asteroid;
    
    grid_world.get_grid(2, 3).unwrap().content = GameObjectTypes::Asteroid;
    grid_world.get_grid(5, 3).unwrap().content = GameObjectTypes::Asteroid;

    grid_world.get_grid(0, 4).unwrap().content = GameObjectTypes::Asteroid;
    grid_world.get_grid(3, 4).unwrap().content = GameObjectTypes::Asteroid;

    grid_world.get_grid(5, 0).unwrap().content = GameObjectTypes::Goal;
}

fn get_grid_world_value(grid_world: &GridWorld) -> Vec<f64>{
    let mut value = Vec::new();

    for _ in 0..(grid_world.y_length * grid_world.y_length){
        value.push(0.0);
    }
    value
}

fn get_grid_world_policy(grid_world: &GridWorld) -> Vec<Moves>{
    let mut value = Vec::new();
    let mut rng = rand::thread_rng();
    

    for _ in 0..(grid_world.y_length * grid_world.y_length){
        value.push(
            match rng.gen_range(0..4){
                0 => Moves::Up,
                1 => Moves::Right,
                2 => Moves::Down,
                3 => Moves::Left,
                _ => Moves::Up,
            }
        );
    }
    value
}