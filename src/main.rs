mod day1;

use aoc24::Day;

fn main() {
    day1::Day1 {}.run();
}

#[cfg(test)]
mod test {
    use super::{day1, Day};

    #[test]
    fn day1() {
        assert_eq!(day1::Day1 {}.test(), (11, 31));
    }
}
