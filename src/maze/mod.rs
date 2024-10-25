use crate::conf::Config;

pub fn run(args: Config) {
    todo!()
}

struct Maze {
    cells: Vec<Cell>,
    width: i32, // assumed to be width in *cells*
    height: i32, // likewise, height in *cells*
    w_actual: i32, // actual dimensions of the maze
    h_actual: i32, // as above
}
impl Maze {
    fn from(width: i32, height: i32) -> Self {
        Self {
            cells: vec![], 
            width, 
            height, 
            w_actual: 2 * width + 1, 
            h_actual: 2 * height + 1
        }
    }
}

struct Cell {
    coord: Coord,
    v: bool, // represents visited status
    b: bool, // represents blocked status
}
impl Cell {
    fn from(coord: Coord, v: bool, b: bool) -> Self{
        Self{coord, v, b}
    }
}

struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn from(x: i32, y: i32) -> Self {
        Self{x, y}
    }
}