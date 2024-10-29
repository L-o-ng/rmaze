use std::cmp::min;

use rand::Rng;

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

    fn build(&mut self, mut cell: Cell) {
        cell.v = true;
        let unv = Self::get_unv_cells(
            &self,
            cell.coord.x, 
            cell.coord.y);
        
        while unv.len() > 0 {
            let t = unv[rand::thread_rng()
                .gen_range(0..unv.len()) as usize
                ];

            if t.v {
                return;
            }

            
        }

        todo!()
    }

    fn set_wall(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, new: bool) {
        let m_x = min(x1, x2) + (x1 - x2).abs() / 2;
        let m_y = min(y1, y2) + (y1 - y2).abs() / 2;
        self.cells[(m_y * self.h_actual + m_x) as usize].v = new;
    }

    fn get_unv_cells(&self, x: i32, y: i32) -> Vec<&Cell> {
        let mut c = vec![];
        if y - 2 >= 0 && !self.cells[((y - 2) * self.h_actual + x) as usize].v {
            c.push(&self.cells[((y - 2) * self.h_actual + x) as usize]);
        }
        if x + 2 < self.w_actual && !self.cells[(y * self.h_actual + x + 2) as usize].v {
            c.push(&self.cells[(y * self.h_actual + x + 2) as usize]);
        }
        if y + 2 < self.h_actual && !self.cells[((y + 2) * self.h_actual + x) as usize].v {
            c.push(&self.cells[((y + 2) * self.h_actual + x) as usize]);
        }
        if x - 2 >= 0 && !self.cells[(y * self.h_actual + x - 2) as usize].v {
            c.push(&self.cells[(y * self.h_actual + x - 2) as usize]);
        }

        c
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