pub struct Day15;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellState {
    Empty,
    Wall,
    Robot,
    Box,
    LBox,
    RBox,
}

impl CellState {
    fn from_char(c: char) -> CellState {
        match c {
            '.' => CellState::Empty,
            '#' => CellState::Wall,
            '@' => CellState::Robot,
            'O' => CellState::Box,
            _ => panic!("Invalid cell state"),
        }
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            CellState::Empty => '.',
            CellState::Wall => '#',
            CellState::Robot => '@',
            CellState::Box => 'O',
            CellState::LBox => '[',
            CellState::RBox => ']',
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
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
}

struct Grid {
    cells: Vec<Vec<CellState>>,
    robot: (i32, i32),
}

impl Grid {
    fn can_move(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        let new_x = x + dx;
        let new_y = y + dy;

        match self.cells[new_y as usize][new_x as usize] {
            CellState::Empty => true,
            CellState::Wall => false,
            CellState::Box => self.can_move(new_x, new_y, dx, dy),
            CellState::LBox => {
                if dx == -1 || dx == 1 {
                    self.can_move(new_x, new_y, dx, dy)
                } else if dy == -1 || dy == 1 {
                    self.can_move(new_x, new_y, dx, dy) && self.can_move(new_x + 1, new_y, dx, dy)
                } else {
                    panic!("Invalid move")
                }
            }
            CellState::RBox => {
                if dx == 1 || dx == -1 {
                    self.can_move(new_x, new_y, dx, dy)
                } else if dy == -1 || dy == 1 {
                    self.can_move(new_x, new_y, dx, dy) && self.can_move(new_x - 1, new_y, dx, dy)
                } else {
                    panic!("Invalid move")
                }
            }
            _ => panic!("Invalid cell state"),
        }
    }

    fn do_move(&mut self, x: i32, y: i32, dx: i32, dy: i32) {
        let new_x = x + dx;
        let new_y = y + dy;

        if dy == -1 || dy == 1 {
            if self.cells[y as usize][x as usize] == CellState::Robot
                || self.cells[y as usize][x as usize] == CellState::LBox
            {
                if self.cells[new_y as usize][new_x as usize] == CellState::RBox {
                    self.do_move(new_x - 1, new_y, dx, dy);
                }
            }

            if self.cells[y as usize][x as usize] == CellState::Robot
                || self.cells[y as usize][x as usize] == CellState::RBox
            {
                if self.cells[new_y as usize][new_x as usize] == CellState::LBox {
                    self.do_move(new_x + 1, new_y, dx, dy);
                }
            }
        }

        match self.cells[new_y as usize][new_x as usize] {
            CellState::Empty => {}
            CellState::Wall => panic!("Shouldn't have moved!"),
            CellState::Box | CellState::LBox | CellState::RBox => {
                self.do_move(new_x, new_y, dx, dy);
            }
            _ => panic!("Invalid cell state"),
        }

        self.cells[new_y as usize][new_x as usize] = self.cells[y as usize][x as usize];

        if self.cells[y as usize][x as usize] == CellState::Robot {
            self.robot = (new_x, new_y);
        }

        self.cells[y as usize][x as usize] = CellState::Empty;
    }

    fn get_sum(&self) -> i32 {
        let mut sum = 0;
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == CellState::Box || *cell == CellState::LBox {
                    sum += 100 * y + x;
                }
            }
        }
        sum as i32
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{}", cell.to_char());
            }
            println!();
        }
    }

    fn double(&self) -> Grid {
        let mut grid = Grid {
            cells: Vec::new(),
            robot: (self.robot.0 * 2, self.robot.1),
        };

        for row in &self.cells {
            let mut new_row = Vec::new();
            for cell in row {
                match cell {
                    CellState::Box => {
                        new_row.push(CellState::LBox);
                        new_row.push(CellState::RBox);
                    }
                    CellState::Robot => {
                        new_row.push(CellState::Robot);
                        new_row.push(CellState::Empty);
                    }
                    _ => {
                        new_row.push(*cell);
                        new_row.push(*cell);
                    }
                }
            }
            grid.cells.push(new_row);
        }

        grid
    }
}

impl aoc24::DayInner<Day15, i32> for Day15 {
    fn day(&self) -> i32 {
        15
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let mut grid = Grid {
            cells: Vec::new(),
            robot: (0, 0),
        };
        let mut in_direction_section = false;

        let mut directions: Vec<Direction> = Vec::new();

        for line in lines {
            if line.is_empty() {
                in_direction_section = true;
                continue;
            }

            if !in_direction_section {
                // Parse the grid
                let mut cells = Vec::new();
                for c in line.chars() {
                    let cell = CellState::from_char(c);
                    cells.push(cell);
                }
                grid.cells.push(cells);
            } else {
                // Parse the directions
                for c in line.chars() {
                    let direction = Direction::from_char(c);
                    directions.push(direction);
                }
            }
        }

        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == CellState::Robot {
                    grid.robot = (x as i32, y as i32);
                }
            }
        }

        let mut double = grid.double();

        for direction in directions {
            let (dx, dy) = match direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            if grid.can_move(grid.robot.0, grid.robot.1, dx, dy) {
                grid.do_move(grid.robot.0, grid.robot.1, dx, dy)
            }

            if double.can_move(double.robot.0, double.robot.1, dx, dy) {
                double.do_move(double.robot.0, double.robot.1, dx, dy)
            }
        }

        let sum1 = grid.get_sum();
        let sum2 = double.get_sum();

        // And we're done!
        (sum1, sum2)
    }
}
