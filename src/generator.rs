use file::{create_file, write_file};
use input::read_input;
use structs::Maze;

mod file;
mod input;
mod structs;

fn edging(maze: &Maze) {
    let width: usize = maze.get_dimensions().0 as usize;
    let height: usize = maze.get_dimensions().1 as usize;
    let mut v: Vec<String> = Vec::with_capacity(height);

    for (y, str) in v.iter_mut().enumerate().take(height) {
        for x in 0..width {
            if y == 0 || y == height || x == 0 || x == width {
                str.push_str(maze.get_wall());
            } else {
                str.push_str(maze.get_path());
            }
        }
    }
    for str in v {
        println!("{str}");
        if let Err(e) = write_file("maze", &str) {
            println!("{e}");
            return;
        }
    }
}

fn create(maze: Maze) {
    if let Err(e) = create_file("maze") {
        println!("{e}");
        return;
    }
    edging(&maze);
}

pub fn generate() -> Result<(), Box<dyn std::error::Error>> {
    let width: u8 = read_input("Enter the width of the maze: ").parse()?;
    let height: u8 = read_input("Enter the height of the maze: ").parse()?;
    let wall = read_input("Enter characters to represent walls: ");
    let path = read_input("Enter characters to represent paths: ");
    match Maze::new(width as i8, height as i8, &wall, &path) {
        Ok(maze) => create(maze),
        Err(e) => println!("{e}"),
    }
    Ok(())
}
