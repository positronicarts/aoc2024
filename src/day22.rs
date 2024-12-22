use std::collections::HashMap;
pub struct Day22;

fn iterate(mut secret: i64) -> i64 {
    secret = (secret * 64) ^ secret;
    secret = secret % 16777216;
    secret = (secret / 32) ^ secret;
    secret = secret % 16777216;
    secret = (secret * 2048) ^ secret;
    secret = secret % 16777216;
    secret
}

impl aoc24::DayInner<Day22, i64> for Day22 {
    fn day(&self) -> i32 {
        22
    }

    fn inner(&self, input: String) -> (i64, i64) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let mut sum = 0;

        let mut prices: Vec<Vec<i64>> = Vec::new();
        let mut deltas: Vec<Vec<i64>> = Vec::new();
        let mut monkey_hashmaps: Vec<HashMap<(i64, i64, i64, i64), i64>> = Vec::new();
        let mut overall_hashmap: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

        for (mm, line) in lines.enumerate() {
            let mut secret = line.parse::<i64>().unwrap();
            let mut monkey_prices = Vec::new();
            let mut monkey_deltas = Vec::new();

            monkey_prices.push(secret % 10);
            monkey_hashmaps.push(HashMap::new());

            for ii in 0..2000 {
                secret = iterate(secret);
                monkey_prices.push(secret % 10);
                monkey_deltas.push(
                    monkey_prices[monkey_prices.len() - 1] - monkey_prices[monkey_prices.len() - 2],
                );
                if ii >= 3 {
                    let mb = secret % 10;
                    let key = (
                        monkey_deltas[monkey_deltas.len() - 4],
                        monkey_deltas[monkey_deltas.len() - 3],
                        monkey_deltas[monkey_deltas.len() - 2],
                        monkey_deltas[monkey_deltas.len() - 1],
                    );
                    if monkey_hashmaps[mm].get(&key).is_none() {
                        monkey_hashmaps[mm].insert(key, mb);
                        if let Some(b) = overall_hashmap.get(&key) {
                            overall_hashmap.insert(key, mb + *b);
                        } else {
                            overall_hashmap.insert(key, mb);
                        }
                    }
                }
            }

            prices.push(monkey_prices);
            deltas.push(monkey_deltas);

            sum += secret;
        }

        let mut best_bananas = 0;

        for d1 in -9..9 {
            for d2 in -9..9 {
                for d3 in -9..9 {
                    for d4 in -9..9 {
                        if let Some(b) = overall_hashmap.get(&(d1, d2, d3, d4)) {
                            if *b > best_bananas {
                                best_bananas = *b;
                            }
                        }
                    }
                }
            }
        }

        // And we're done!
        (sum, best_bananas)
    }
}
