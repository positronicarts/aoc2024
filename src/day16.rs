use std::{collections::HashSet, vec};

pub struct Day16;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellState {
    Empty,
    Wall,
    Start,
    End,
    Route,
}

impl CellState {
    fn from_char(c: char) -> CellState {
        match c {
            '.' => CellState::Empty,
            '#' => CellState::Wall,
            'S' => CellState::Start,
            'E' => CellState::End,
            _ => panic!("Invalid cell state"),
        }
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            CellState::Empty => '.',
            CellState::Wall => '#',
            CellState::Start => 'S',
            CellState::End => 'E',
            CellState::Route => 'O',
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[allow(dead_code)]
    fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vector {
    location: (i32, i32),
    direction: Direction,
    cost: Option<i32>,
    routes: Vec<(i32, i32)>,
}

struct Grid {
    cells: Vec<Vec<CellState>>,
}

impl Grid {
    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{}", cell.to_char());
            }
            println!();
        }
    }
}

impl aoc24::DayInner<Day16, i32> for Day16 {
    fn day(&self) -> i32 {
        16
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let mut grid = Grid { cells: Vec::new() };

        for line in lines {
            // Parse the grid
            let mut cells = Vec::new();
            for c in line.chars() {
                let cell = CellState::from_char(c);
                cells.push(cell);
            }
            grid.cells.push(cells);
        }

        let mut end = (0, 0);
        let mut unvisited_set = Vec::new();
        let mut visited_set = Vec::new();

        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == CellState::Empty || *cell == CellState::End {
                    for d in [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .iter()
                    {
                        unvisited_set.push(Vector {
                            location: (x as i32, y as i32),
                            direction: *d,
                            cost: None,
                            routes: Vec::new(),
                        });
                    }
                }

                if *cell == CellState::End {
                    end = (x as i32, y as i32);
                }

                if *cell == CellState::Start {
                    // location.location = (x as i32, y as i32);

                    unvisited_set.push(Vector {
                        location: (x as i32, y as i32),
                        direction: Direction::Right,
                        cost: Some(0),
                        routes: vec![(x as i32, y as i32)],
                    });

                    unvisited_set.push(Vector {
                        location: (x as i32, y as i32),
                        direction: Direction::Up,
                        cost: None,
                        routes: Vec::new(),
                    });
                    unvisited_set.push(Vector {
                        location: (x as i32, y as i32),
                        direction: Direction::Down,
                        cost: None,
                        routes: Vec::new(),
                    });
                    unvisited_set.push(Vector {
                        location: (x as i32, y as i32),
                        direction: Direction::Left,
                        cost: None,
                        routes: Vec::new(),
                    });
                }
            }
        }

        let mut cost1: i32 = 0;

        // Implement Dijkstra's algorithm
        loop {
            // Find the element with the lowest cost
            let mut lowest_cost = None;
            let mut lowest_vector = None;
            let mut lowest_index = 0;

            for (index, vector) in unvisited_set.iter().enumerate() {
                if let Some(cost) = vector.cost {
                    if lowest_cost.is_none() || cost < lowest_cost.unwrap() {
                        lowest_cost = Some(cost);
                        lowest_vector = Some(vector.clone());
                        lowest_index = index;
                    }
                }
            }

            if lowest_vector.is_none() {
                break;
            }

            let vector = lowest_vector.unwrap();

            if vector.location == end {
                let cost = vector.cost.unwrap();
                if cost < cost1 || cost1 == 0 {
                    cost1 = cost;
                }
            }

            // Update the adjacent costs
            let (dx, dy) = vector.direction.to_delta();
            let new_location = (vector.location.0 + dx, vector.location.1 + dy);

            for v in unvisited_set.iter_mut() {
                if v.location == new_location && v.direction == vector.direction {
                    let new_cost = vector.cost.unwrap() + 1;
                    if v.cost.is_none() || new_cost <= v.cost.unwrap() {
                        // If it's a strictly better route, clear current routes
                        if v.cost.is_some() && v.cost.unwrap() > new_cost {
                            v.routes.clear();
                        }

                        (*v).cost = Some(new_cost);

                        if !v.routes.contains(&v.location) {
                            v.routes.push(v.location);
                        }
                        for r in vector.routes.iter() {
                            if !v.routes.contains(r) {
                                v.routes.push(*r);
                            }
                        }
                    }
                }

                if v.location == vector.location {
                    if v.direction.opposite() != vector.direction && v.direction != vector.direction
                    {
                        let new_cost = vector.cost.unwrap() + 1000;
                        if v.cost.is_none() || new_cost <= v.cost.unwrap() {
                            if v.cost.is_some() && v.cost.unwrap() > new_cost {
                                v.routes.clear();
                            }

                            (*v).cost = Some(new_cost);

                            for r in vector.routes.iter() {
                                if !v.routes.contains(r) {
                                    v.routes.push(*r);
                                }
                            }
                        }
                    }
                }
            }

            visited_set.push(vector);
            unvisited_set.remove(lowest_index);
        }

        // Look at end points in the visited set
        let mut visited_locations = HashSet::new();
        for v in visited_set.iter() {
            if v.location == end && v.cost == Some(cost1) {

                for r in v.routes.iter() {
                    visited_locations.insert(*r);
                    grid.cells[r.1 as usize][r.0 as usize] = CellState::Route;
                }
            }
        }

        grid.print();

        // And we're done!
        (cost1, visited_locations.len() as i32)
    }
}
