mod day1;
mod day2;
mod day3;

use aoc24::Day;

fn main() {
    // day1::Day1 {}.run();
    // day2::Day2 {}.run();
    day3::Day3 {}.run();
}

#[cfg(test)]
mod test {
    use super::{day1, day2, day3, Day};

    #[test]
    fn day1() {
        assert_eq!(day1::Day1 {}.test(), (11, 31));
    }

    #[test]
    fn day2() {
        assert_eq!(day2::Day2 {}.test(), (2, 4));
    }

    #[test]
    fn day3() {
        assert_eq!(day3::Day3 {}.test(), (161, 48));
    }
}
