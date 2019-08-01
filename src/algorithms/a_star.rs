use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

use log::trace;

#[repr(C)]
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
    pub cols: i32,
    pub rows: i32,
    pub cells: Vec<GridCell>,
}

///
/// https://stackoverflow.com/questions/39949939/how-can-i-implement-a-min-heap-of-f64-with-rusts
/// -binaryheap
#[derive(Debug, Copy, Clone, PartialEq)]
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

        let cells = (0..cols * rows).map(|_| GridCell::Empty).collect();

        Grid { cols, rows, cells }
    }

    /// Get cell at position 'pos' unchecked.
    ///
    pub fn get_cell(&self, pos: Point) -> GridCell {
        self.cells[self.get_index(pos)]
    }

    pub fn set_cell(&mut self, pos: Point, value: GridCell) {
        let index = (pos.x + self.cols * pos.y) as usize;
        self.cells[index] = value;
    }

    /// Calculate the index of 'pos'
    ///
    pub fn get_index(&self, pos: Point) -> usize {
        (pos.x + self.cols * pos.y) as usize
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

        let mut parents = HashMap::new();

        while let Some(State { g, position, .. }) = heap.pop() {
            trace!("Visiting {:?}...", position);

            if position == goal {
                trace!("Bingo!!! {:?}", position);
                return Ok(self.final_path(&parents, position, start));
            }

            for pos in self.get_adjacent(position) {
                if closed.contains(&position) {
                    continue;
                }

                let new_g = g + 1.0; // distance between successor and q

                //                if  { }
                //                if let Some(n) = heap.iter().find(|&s| s.position == pos) {
                //                    if new_g < n.g {}
                //                } else {
                //
                //                }

                let new_state = State {
                    g: new_g,
                    f: new_g + Self::calc_heuristic(pos, goal), // dist between q and goal
                    position: pos,
                };

                heap.push(new_state);
                parents.entry(pos).or_insert(position);

                trace!(" --> {:?}", new_state);
            }

            closed.insert(position);
        }

        Err(String::from("Could not find a path!!!"))
    }

    pub fn final_path(
        &self,
        parents: &HashMap<Point, Point>,
        point: Point,
        start: Point,
    ) -> Vec<Point> {
        let mut path = vec![];
        let mut current = point;

        while let Some(&parent) = parents.get(&current) {
            if current == start {
                break;
            }

            current = parent;
            path.push(current);
            trace!("{:?}", current)
        }

        path.insert(0, point);
        path.reverse();
        path
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
            .filter(|&&pos| self.valid_pos(pos))
            .cloned()
            .collect()
    }

    pub fn valid_pos(&self, pos: Point) -> bool {
        !(pos.x < 0 || pos.x >= self.cols || pos.y < 0 || pos.y >= self.rows)
    }

    /// Here defines the Heuristic function.
    ///
    pub fn calc_heuristic(a: Point, b: Point) -> f64 {
        f64::from((a.x - b.x).pow(2) + (a.y - b.y).pow(2))
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
            writeln!(f)?;
        }
        Ok(())
    }
}

type PathFindingResult = Result<Vec<Point>, String>;
