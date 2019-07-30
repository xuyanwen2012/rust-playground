use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridCell {
    Empty = 0,
    Occupied = 1,
}

#[derive(Debug)]
pub struct Grid {
    cols: i32,
    rows: i32,
    cells: Vec<GridCell>,
}

///
/// https://stackoverflow.com/questions/39949939/how-can-i-implement-a-min-heap-of-f64-with-rusts
/// -binaryheap
#[derive(Copy, Clone, PartialEq)]
struct State {
    f: f64,
    position: Point,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Grid {
    /// Constructor
    ///
    pub fn new() -> Self {
        let cols: i32 = 16;
        let rows: i32 = 16;

        let cells = (0..cols * rows).map(|i| GridCell::Empty).collect();

        Grid { cols, rows, cells }
    }

    /// Get cell at position 'pos' unchecked.
    ///
    pub fn get_cell(&self, pos: Point) -> GridCell {
        self.cells[(pos.x + self.cols * pos.y) as usize]
    }

    /// A Star path finding from 'start' to 'goal'
    ///
    pub fn shortest_path(&self, start: Point, goal: Point) -> PathFindingResult {
        let mut closed: Vec<Point> = vec![];
        let mut heap = BinaryHeap::new();
        heap.push(State {
            f: 0.0,
            position: start,
        });

        while let Some(State { f, position }) = heap.pop() {
            if position == goal {
                println!("Bingo!!!");
                return Ok(vec![]);
            }
        }

        Err(String::from("Could not find a path!!!"))
    }

    pub fn get_adjacent(&self, pos: Point) -> Vec<Point> {
        let Point { x, y } = pos;

        //        let adjacent = (-1..2).map(|i| Point { x: x + i, y: 0 }).collect();
        let adjacent = vec![
            Point::new(x - 1, y - 1),
            Point::new(x, y - 1),
            Point::new(x + 1, y - 1),
            Point::new(x - 1, y),
            Point::new(x + 1, y),
            Point::new(x - 1, y + 1),
            Point::new(x, y + 1),
            Point::new(x + 1, y + 1),
        ];

        adjacent
            .iter()
            .filter(|pos| self.valid_pos(pos))
            .cloned()
            .collect()
    }

    fn valid_pos(&self, pos: &Point) -> bool {
        !(pos.x < 0 || pos.x >= self.cols || pos.y < 0 || pos.y >= self.rows)
    }
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
