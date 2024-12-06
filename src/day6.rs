use std::collections::HashSet;

pub struct Day6;

#[derive(PartialEq, Eq, Debug, Clone)]
enum GridStatus {
    Empty,
    Occupied,
    Visited(HashSet<Direction>),
}

impl GridStatus {
    fn is_visited(&self) -> bool {
        match self {
            GridStatus::Visited(_) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct LocationWithDirection {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<GridStatus>>,
}

impl Grid {
    fn move_location(
        &mut self,
        location: LocationWithDirection,
    ) -> (Option<LocationWithDirection>, bool) {
        let x = location.x;
        let y = location.y;
        let direction = location.direction;

        match direction {
            Direction::Up => {
                if y == 0 {
                    (None, false)
                } else {
                    match &mut self.grid[(y - 1) as usize][x as usize] {
                        GridStatus::Empty => {
                            self.grid[(y - 1) as usize][x as usize] =
                                GridStatus::Visited(HashSet::from([Direction::Up]));
                            (
                                Some(LocationWithDirection {
                                    x: x,
                                    y: y - 1,
                                    direction: direction,
                                }),
                                false,
                            )
                        }
                        GridStatus::Occupied => (
                            Some(LocationWithDirection {
                                x: x,
                                y: y,
                                direction: direction.turn_right(),
                            }),
                            false,
                        ),
                        GridStatus::Visited(directions) => {
                            if directions.contains(&Direction::Up) {
                                (None, true)
                            } else {
                                directions.insert(Direction::Up);
                                (
                                    Some(LocationWithDirection {
                                        x: x,
                                        y: y - 1,
                                        direction: direction,
                                    }),
                                    false,
                                )
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                if y == self.grid.len() as i32 - 1 {
                    (None, false)
                } else {
                    match &mut self.grid[(y + 1) as usize][x as usize] {
                        GridStatus::Empty => {
                            self.grid[(y + 1) as usize][x as usize] =
                                GridStatus::Visited(HashSet::from([Direction::Down]));
                            (
                                Some(LocationWithDirection {
                                    x: x,
                                    y: y + 1,
                                    direction: direction,
                                }),
                                false,
                            )
                        }
                        GridStatus::Occupied => (
                            Some(LocationWithDirection {
                                x: x,
                                y: y,
                                direction: direction.turn_right(),
                            }),
                            false,
                        ),
                        GridStatus::Visited(directions) => {
                            if directions.contains(&Direction::Down) {
                                (None, true)
                            } else {
                                directions.insert(Direction::Down);
                                (
                                    Some(LocationWithDirection {
                                        x: x,
                                        y: y + 1,
                                        direction: direction,
                                    }),
                                    false,
                                )
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                if x == 0 {
                    (None, false)
                } else {
                    match &mut self.grid[y as usize][(x - 1) as usize] {
                        GridStatus::Empty => {
                            self.grid[y as usize][(x - 1) as usize] =
                                GridStatus::Visited(HashSet::from([Direction::Left]));
                            (
                                Some(LocationWithDirection {
                                    x: x - 1,
                                    y: y,
                                    direction: direction,
                                }),
                                false,
                            )
                        }
                        GridStatus::Occupied => (
                            Some(LocationWithDirection {
                                x: x,
                                y: y,
                                direction: direction.turn_right(),
                            }),
                            false,
                        ),
                        GridStatus::Visited(directions) => {
                            if directions.contains(&Direction::Left) {
                                (None, true)
                            } else {
                                directions.insert(Direction::Left);
                                (
                                    Some(LocationWithDirection {
                                        x: x - 1,
                                        y: y,
                                        direction: direction,
                                    }),
                                    false,
                                )
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                if x == self.grid[0].len() as i32 - 1 {
                    (None, false)
                } else {
                    match &mut self.grid[y as usize][(x + 1) as usize] {
                        GridStatus::Empty => {
                            self.grid[y as usize][(x + 1) as usize] =
                                GridStatus::Visited(HashSet::from([Direction::Right]));
                            (
                                Some(LocationWithDirection {
                                    x: x + 1,
                                    y: y,
                                    direction: direction,
                                }),
                                false,
                            )
                        }
                        GridStatus::Occupied => (
                            Some(LocationWithDirection {
                                x: x,
                                y: y,
                                direction: direction.turn_right(),
                            }),
                            false,
                        ),
                        GridStatus::Visited(directions) => {
                            if directions.contains(&Direction::Right) {
                                (None, true)
                            } else {
                                directions.insert(Direction::Right);
                                (
                                    Some(LocationWithDirection {
                                        x: x + 1,
                                        y: y,
                                        direction: direction,
                                    }),
                                    false,
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

impl aoc24::DayInner<Day6, i32> for Day6 {
    fn day(&self) -> i32 {
        6
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();
        let starting_grid: Grid = Grid {
            grid: lines
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => GridStatus::Empty,
                            '#' => GridStatus::Occupied,
                            '^' => GridStatus::Visited(HashSet::from([Direction::Up])),
                            _ => panic!("Invalid character in input"),
                        })
                        .collect()
                })
                .collect(),
        };

        let mut starting_location = None;

        // Infer the starting location and direction
        for (y, row) in starting_grid.grid.iter().enumerate() {
            for (x, status) in row.iter().enumerate() {
                if *status == GridStatus::Visited(HashSet::from([Direction::Up])) {
                    starting_location = Some(LocationWithDirection {
                        x: x as i32,
                        y: y as i32,
                        direction: Direction::Up,
                    });
                    break;
                }
            }
        }

        let starting_location_unwrapped = starting_location.unwrap();
        let sx = starting_location_unwrapped.x;
        let sy = starting_location_unwrapped.y;
        let sd = starting_location_unwrapped.direction;

        let mut location = Some(LocationWithDirection {
            x: sx,
            y: sy,
            direction: sd.clone(),
        });
        let mut grid = starting_grid.clone();
        while location.is_some() {
            (location, _) = grid.move_location(location.unwrap());
        }

        // Count the number of visited locations
        let visited = grid
            .grid
            .iter()
            .map(|row| row.iter().filter(|status| status.is_visited()).count())
            .sum::<usize>() as i32;

        let mut loops = 0;
        for ii in 0..starting_grid.grid.len() {
            for jj in 0..starting_grid.grid[0].len() {
                match starting_grid.grid[jj][ii] {
                    GridStatus::Empty => {
                        let mut grid = starting_grid.clone();
                        grid.grid[jj][ii] = GridStatus::Occupied;

                        let mut location = Some(LocationWithDirection {
                            x: sx,
                            y: sy,
                            direction: sd.clone(),
                        });
                        let mut looped = false;
                        while location.is_some() {
                            (location, looped) = grid.move_location(location.unwrap());
                        }
                        if looped {
                            loops += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        // And we're done!
        (visited, loops)
    }
}
