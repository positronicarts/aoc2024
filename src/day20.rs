// use std::collections::HashSet;

use std::collections::{HashMap, HashSet};

pub struct Day20;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellState {
    Empty,
    Wall,
    Start,
    End,
    // Route,
}

impl CellState {
    #[allow(dead_code)]
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
            // CellState::Route => 'O',
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

    #[allow(dead_code)]
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
    // direction: Direction,
    cost: Option<i32>,
    // routes: Vec<(i32, i32)>,
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<(CellState, i32)>>,
}

impl Grid {
    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{}", cell.0.to_char());
            }
            println!();
        }
    }
}

impl aoc24::DayInner<Day20, i32> for Day20 {
    fn day(&self) -> i32 {
        20
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines: Vec<_> = input.lines().collect();

        let test = lines[0] == "###############";
        let mut og_grid = Grid { cells: Vec::new() };

        let mut start = (0, 0);
        let mut end = (0, 0);

        for line in lines {
            // Parse the grid
            let mut cells = Vec::new();
            for c in line.chars() {
                let cell = CellState::from_char(c);
                if cell == CellState::Start {
                    start = (cells.len() as i32, og_grid.cells.len() as i32);
                } else if cell == CellState::End {
                    end = (cells.len() as i32, og_grid.cells.len() as i32);
                }
                cells.push((cell, 99999999));
            }
            og_grid.cells.push(cells);
        }

        let mut reduced_costs_1 = Vec::new();
        let mut reduced_costs_2 = Vec::new();

        let threshold = if test { (0, 50) } else { (100, 100) };

        let forward = dijkstra(start, og_grid.clone());
        let backward = dijkstra(end, og_grid.clone());

        let best_cost = forward.cells[end.1 as usize][end.0 as usize].1;

        for y in 1..og_grid.cells.len() - 1 {
            for x in 1..og_grid.cells[y].len() - 1 {
                if og_grid.cells[y][x].0 != CellState::Wall {
                    // Check how many valid cheats there are
                    for yy in 1..og_grid.cells.len() - 1 {
                        for xx in 1..og_grid.cells[y].len() - 1 {
                            if og_grid.cells[yy][xx].0 == CellState::Wall {
                                continue;
                            }

                            let manhattan = ((yy as i32 - y as i32).abs()
                                + (xx as i32 - x as i32).abs())
                                as i32;
                            let cheat_cost =
                                forward.cells[y][x].1 + manhattan + backward.cells[yy][xx].1;

                            if manhattan <= 2 {
                                // Valid for part 1
                                if cheat_cost < best_cost && cheat_cost <= (best_cost - threshold.0)
                                {
                                    reduced_costs_1.push(best_cost - cheat_cost);
                                }
                            }
                            if manhattan <= 20 {
                                // Valid for part 2
                                if cheat_cost < best_cost && cheat_cost <= (best_cost - threshold.1)
                                {
                                    reduced_costs_2.push(best_cost - cheat_cost);
                                }
                            }
                        }
                    }
                }
            }
        }

        // And we're done!
        (reduced_costs_1.len() as i32, reduced_costs_2.len() as i32)
    }
}

fn dijkstra(start: (i32, i32), mut grid: Grid) -> Grid {
    let mut unvisited_set: HashSet<(i32, i32)> = HashSet::new();
    let mut boundary_map: HashMap<(i32, i32), i32> = HashMap::new();

    for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.0 == CellState::Empty || cell.0 == CellState::End {
                unvisited_set.insert((x as i32, y as i32));
            }
        }
    }

    boundary_map.insert(start.clone(), 0);

    // Implement Dijkstra's algorithm
    while !boundary_map.is_empty() {
        // Find the element with the lowest cost
        let mut lowest_cost: Option<i32> = None;
        let mut lowest_vector: Option<(i32, i32)> = None;
        for (k, v) in boundary_map.iter() {
            if lowest_cost.is_none() || v < &lowest_cost.unwrap() {
                lowest_cost = Some(*v);
                lowest_vector = Some(*k);
            }
        }

        if lowest_vector.is_none() {
            break;
        }

        let cost_to_here = lowest_cost.unwrap();
        let vector = lowest_vector.unwrap();
        grid.cells[vector.1 as usize][vector.0 as usize].1 = cost_to_here;

        // Update the adjacent costs
        let new_cost = cost_to_here + 1;
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        {
            let (dx, dy) = d.to_delta();
            let new_location = (vector.0 + dx, vector.1 + dy);

            if !unvisited_set.contains(&new_location) {
                continue;
            }

            if boundary_map.contains_key(&new_location) {
                if new_cost < boundary_map[&new_location] {
                    boundary_map.insert(new_location, new_cost);
                }
            } else {
                boundary_map.insert(new_location, new_cost);
            }
        }

        unvisited_set.remove(&vector);
        boundary_map.remove(&vector);
    }

    return grid;
    // panic!("No path found");
}
