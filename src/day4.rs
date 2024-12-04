pub struct Day4;

fn check_match(grid: &Vec<Vec<char>>, start_x: i32, start_y: i32, dx: i32, dy: i32) -> bool {
    grid[start_y as usize][start_x as usize] == 'X'
        && grid[(start_y + dy) as usize][(start_x + dx) as usize] == 'M'
        && grid[(start_y + dy * 2) as usize][(start_x + dx * 2) as usize] == 'A'
        && grid[(start_y + dy * 3) as usize][(start_x + dx * 3) as usize] == 'S'
}

fn check_xmas(grid: &Vec<Vec<char>>, start_x: i32, start_y: i32) -> bool {
    if grid[start_y as usize][start_x as usize] != 'A' {
        return false;
    }

    if grid[(start_y - 1) as usize][(start_x - 1) as usize] == 'M'
        && grid[(start_y + 1) as usize][(start_x + 1) as usize] == 'S'
        && grid[(start_y + 1) as usize][(start_x - 1) as usize] == 'M'
        && grid[(start_y - 1) as usize][(start_x + 1) as usize] == 'S'
    {
        return true;
    }

    if grid[(start_y - 1) as usize][(start_x - 1) as usize] == 'M'
        && grid[(start_y + 1) as usize][(start_x + 1) as usize] == 'S'
        && grid[(start_y - 1) as usize][(start_x + 1) as usize] == 'M'
        && grid[(start_y + 1) as usize][(start_x - 1) as usize] == 'S'
    {
        return true;
    }

    if grid[(start_y - 1) as usize][(start_x - 1) as usize] == 'S'
        && grid[(start_y + 1) as usize][(start_x + 1) as usize] == 'M'
        && grid[(start_y + 1) as usize][(start_x - 1) as usize] == 'M'
        && grid[(start_y - 1) as usize][(start_x + 1) as usize] == 'S'
    {
        return true;
    }

    if grid[(start_y - 1) as usize][(start_x - 1) as usize] == 'S'
        && grid[(start_y + 1) as usize][(start_x + 1) as usize] == 'M'
        && grid[(start_y - 1) as usize][(start_x + 1) as usize] == 'M'
        && grid[(start_y + 1) as usize][(start_x - 1) as usize] == 'S'
    {
        return true;
    }

    return false;
}

impl aoc24::DayInner<Day4, i32> for Day4 {
    fn day(&self) -> i32 {
        4
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut xmas_count = 0;
        let mut x_mas_count = 0;

        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let width = grid[0].len();
        let height = grid.len();

        // Loop in each direction
        for yy in 0..height {
            for xx in 0..width - 3 {
                if check_match(&grid, xx as i32, yy as i32, 1, 0) {
                    xmas_count += 1;
                }
                if check_match(&grid, (width - 1 - xx) as i32, yy as i32, -1, 0) {
                    xmas_count += 1;
                }
            }
        }

        for yy in 0..height - 3 {
            for xx in 0..width {
                if check_match(&grid, xx as i32, yy as i32, 0, 1) {
                    xmas_count += 1;
                }
                if check_match(&grid, xx as i32, (height - 1 - yy) as i32, 0, -1) {
                    xmas_count += 1;
                }
            }
        }

        for yy in 0..height - 3 {
            for xx in 0..width - 3 {
                if check_match(&grid, xx as i32, yy as i32, 1, 1) {
                    xmas_count += 1;
                }
                if check_match(
                    &grid,
                    (width - 1 - xx) as i32,
                    (height - 1 - yy) as i32,
                    -1,
                    -1,
                ) {
                    xmas_count += 1;
                }
                if check_match(&grid, xx as i32, (height - 1 - yy) as i32, 1, -1) {
                    xmas_count += 1;
                }
                if check_match(&grid, (width - 1 - xx) as i32, yy as i32, -1, 1) {
                    xmas_count += 1;
                }
            }
        }

        for yy in 1..height - 1 {
            for xx in 1..width - 1 {
                if check_xmas(&grid, xx as i32, yy as i32) {
                    x_mas_count += 1;
                }
            }
        }
        // And we're done!
        (xmas_count, x_mas_count)
    }
}
