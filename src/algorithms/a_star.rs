use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt;

//#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Construct a new point, using the provided values.
    #[inline]
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
    g: f64,
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
        let mut closed = HashSet::new(); // should i use with_capacity?
        let mut heap = BinaryHeap::new(); // same here
        heap.push(State {
            f: 0.0,
            g: 0.0,
            position: start,
        });

        while let Some(State { f, g, position }) = heap.pop() {
            if position == goal {
                println!("Bingo!!!");
                return Ok(vec![]);
            }

            println!("Visiting {:?}...", position);

            for pos in self.get_adjacent(position) {
                if closed.contains(&position) {
                    continue;
                }

                let new_state = State {
                    f: f + 5.0,
                    g: g + 1.0,
                    position: pos,
                };

                if heap.iter().all(|&n| n.position != pos) {
                    heap.push(new_state)
                } else if new_state.f >= g {
                    continue;
                }

                println!(" --> {:?}", pos)
            }

            closed.insert(position);
        }

        Err(String::from("Could not find a path!!!"))
    }

    /// Get a list of neighbors at position 'pos'
    ///
    pub fn get_adjacent(&self, pos: Point) -> Vec<Point> {
        let Point { x, y } = pos;

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

    /// Here defines the Heuristic function.
    fn calc_heuristic(start: Point, end: Point) -> f64 {
        0.0
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
