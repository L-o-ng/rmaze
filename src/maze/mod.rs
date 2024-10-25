use crate::conf::Config;

pub fn run(args: Config) {
    let m = Maze::from(
        args.width.parse::<i32>().unwrap(),
        args.height.parse::<i32>().unwrap());

    m.init();

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

    fn init(mut self) -> Self {
        for y in 0..self.h_actual {
            for x in 0..self.w_actual {
                if y % 2 == 0 && x % 2 == 0 {
                    self.cells.append(&mut vec![Cell::from(x, y, false, false)]);
                }
                else {
                    self.cells.append(&mut vec![Cell::from(x, y, false, true)]);
                }
            }
        }
        self
    }
}

struct Cell {
    coord: Coord,
    v: bool, // represents visited status
    b: bool, // represents blocked status
}
impl Cell {
    fn from(x: i32, y: i32, v: bool, b: bool) -> Self{
        Self{coord: Coord::from(x, y), v, b}
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