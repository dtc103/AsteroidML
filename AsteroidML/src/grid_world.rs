use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WorldObjectTypes{
    Asteroid,
    Empty,
    Goal
}

#[derive(Debug, PartialEq, Eq)]
pub enum Moves{
    Up,
    UpJump,
    Down,
    DownJump,
    Left, 
    LeftJump,
    Right,
    RightJump
}

#[derive(Debug)]
pub struct GridWorld{
    pub x_length: u32,
    pub y_length: u32,
    pub grids: Vec<WorldObjectTypes>,
}
impl GridWorld{
    pub fn new(x_size: u32, y_size: u32) -> Self{
        let mut tmp_grid = Vec::new();

        for y in 0..y_size{
            for x in 0..x_size{
                tmp_grid.push(WorldObjectTypes::Empty);
            }
        }
        GridWorld{x_length: x_size, y_length: y_size, grids: tmp_grid}
    }
    pub fn get_grid(&mut self, x: usize, y: usize) -> Result<&WorldObjectTypes, String>{
        if x >= self.x_length as usize || y >= self.y_length as usize{
            return Err(String::from("dimensions do not fit"));
        }

        Ok(&self.grids[(x + y * self.x_length as usize) as usize])
    }
    pub fn set_grid(&mut self, x: usize, y: usize, game_object: WorldObjectTypes) -> Result<(), String>{
        if x >= self.x_length as usize || y >= self.y_length as usize{
            return Err(String::from("dimensions do not fit"));
        }

        self.grids[(x + y * self.x_length as usize) as usize] = game_object;

        Ok(())
    }
}
impl fmt::Display for GridWorld{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        for y in 0..self.y_length{
            for x in 0..self.x_length{
                write!(f, "{:?}\t", self.grids[(x + y*self.x_length) as usize]);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
