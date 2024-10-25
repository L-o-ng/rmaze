pub struct Maze {
    cells: Vec<Cell>,
    width: i32, // assumed to be width in *cells*
    height: i32, // likewise, height in *cells*
    w_actual: i32, // actual dimensions of the maze
    h_actual: i32, // as above
}

struct Cell {
    coord: Coord,
    v: bool, // represents visited status
    b: bool, // represents blocked status
}

struct Coord {
    x: i32,
    y: i32,
}