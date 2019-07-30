use std::fmt;

pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridCell {
    Empty,
    Occupied,
}

#[derive(Debug)]
pub struct Grid {
    cols: u32,
    rows: u32,
    cells: Vec<GridCell>,
}

impl Grid {
    pub fn new() -> Self {
        let cols: u32 = 32;
        let rows: u32 = 32;

        let cells = (0..cols * rows).map(|i| GridCell::Empty).collect();

        Grid { cols, rows, cells }
    }

    //    pub fn get_node(&self, x: u32, y: u32) -> GridCell {
    //        self.cells[(x * self.rows + y) as usize]
    //    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for line in self.cells.as_slice().chunks(self.cols as usize) {
            for &cell in line {
                let symbol = if cell == GridCell::Empty {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

type PathFindingResult = Result<Vec<Point>, String>;

pub fn search(start: Point, end: Point) -> PathFindingResult {
    let mut open: Vec<Point> = vec![];
    let mut closed: Vec<Point> = vec![];
    Err(String::from("Error"))
}
