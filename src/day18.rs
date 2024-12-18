// use std::collections::HashSet;

pub struct Day18;

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
    routes: Vec<(i32, i32)>,
}

#[derive(Clone)]
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

impl aoc24::DayInner<Day18, i32> for Day18 {
    fn day(&self) -> i32 {
        18
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines: Vec<_> = input.lines().collect();

        let mut og_grid = Grid { cells: Vec::new() };

        let mut test = false;
        if lines[0] == "5,4" {
            test = true;
        }

        let width: i32;
        let height: i32;
        let count: i32;
        if test {
            width = 7;
            height = 7;
            count = 12;
        } else {
            width = 71;
            height = 71;
            count = 1024;
        }

        for _ in 0..height {
            let mut cells = Vec::new();
            for _ in 0..width {
                cells.push(CellState::Empty);
            }
            og_grid.cells.push(cells);
        }

        let coords: Vec<Vec<i32>> = lines
            .iter()
            .map(|line| line.split(",").map(|d| d.parse().unwrap()).collect())
            .collect();

        let end = (width - 1, height - 1);
        og_grid.cells[end.1 as usize][end.0 as usize] = CellState::End;
        og_grid.cells[0][0] = CellState::Start;
        let mut unvisited_set = Vec::new();
        let mut costs = Vec::new();
        let mut latest_best = Vec::new();

        for go in count..coords.len() as i32 {
            if go > count
                && (!latest_best
                    .contains(&(coords[(go - 1) as usize][0], coords[(go - 1) as usize][1])))
            {
                continue;
            }

            let mut grid = og_grid.clone();

            for ii in 0..go {
                let x = coords[ii as usize][0];
                let y = coords[ii as usize][1];
                grid.cells[y as usize][x as usize] = CellState::Wall;
            }

            for (y, row) in grid.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == CellState::Empty || *cell == CellState::End {
                        unvisited_set.push(Vector {
                            location: (x as i32, y as i32),
                            cost: None,
                            routes: Vec::new(),
                        });
                    }
                }
            }

            unvisited_set.push(Vector {
                location: (0, 0),
                cost: Some(0),
                routes: Vec::new(),
            });
            let mut visited_set = Vec::new();

            let mut best_end_cost: i32 = 0;
            let mut stuck = false;

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
                    if best_end_cost == 0 {
                        stuck = true;
                    }
                    break;
                }

                let vector = lowest_vector.unwrap();

                if vector.location == end {
                    let cost = vector.cost.unwrap();
                    if cost < best_end_cost || best_end_cost == 0 {
                        best_end_cost = cost;
                    }
                }

                // Update the adjacent costs
                for d in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .iter()
                {
                    let (dx, dy) = d.to_delta();
                    let new_location = (vector.location.0 + dx, vector.location.1 + dy);

                    for v in unvisited_set.iter_mut() {
                        if v.location == new_location {
                            let new_cost = vector.cost.unwrap() + 1;
                            if v.cost.is_none() || new_cost <= v.cost.unwrap() {
                                (*v).cost = Some(new_cost);
                                (*v).routes = vector.routes.clone();
                                (*v).routes.push(vector.location);
                            }
                        }
                    }
                }

                visited_set.push(vector);
                unvisited_set.remove(lowest_index);
            }

            if stuck {
                println!("Stuck! {:?}", coords[(go - 1) as usize]);
                break;
            }
            costs.push(best_end_cost);

            for v in visited_set.iter() {
                if v.location == end {
                    latest_best = v.routes.clone();
                    break;
                }
            }
        }

        // And we're done!
        (costs[0], 0)
    }
}
