use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day23;

impl aoc24::DayInner<Day23, i32> for Day23 {
    fn day(&self) -> i32 {
        23
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let mut neighbours: HashMap<(char, char), HashSet<(char, char)>> = HashMap::new();

        for line in lines {
            let chars = line.chars().collect::<Vec<char>>();
            let v1 = (chars[0], chars[1]);
            let v2 = (chars[3], chars[4]);

            let set1 = neighbours.entry(v1).or_insert(HashSet::new());
            set1.insert(v2);
            let set2 = neighbours.entry(v2).or_insert(HashSet::new());
            set2.insert(v1);
        }

        // Part 1 - count triples
        let mut triples = HashSet::new();

        for (k, v) in neighbours.iter() {
            if k.0 == 't' {
                for n1 in v.iter() {
                    for n2 in neighbours[n1].iter() {
                        if n2 == k {
                            continue;
                        }
                        if v.contains(n2) {
                            // Found a triple
                            let string1 = format!("{}{}", k.0, k.1);
                            let string2 = format!("{}{}", n1.0, n1.1);
                            let string3 = format!("{}{}", n2.0, n2.1);

                            let mut strings = vec![string1, string2, string3];
                            strings.sort();
                            triples.insert(strings);
                        }
                    }
                }
            }
        }

        // Part 2 - fid maximal interconnected set
        let mut winning = 0;

        for (k, v) in neighbours.iter() {
            let mut candidate_set = v.clone();
            candidate_set.insert(*k);

            for ll in (winning + 1)..candidate_set.len() {
                for subset in candidate_set.iter().combinations(ll) {
                    let mut candidate_subset: HashSet<(char, char)> = HashSet::new();
                    for c in subset.iter() {
                        candidate_subset.insert(**c);
                    }

                    if candidate_subset.len() <= winning {
                        continue;
                    }

                    let mut valid = true;
                    for n in candidate_subset.iter() {
                        for n2 in candidate_subset.iter() {
                            if n != n2 && !neighbours[n].contains(n2) {
                                valid = false;
                                break;
                            }
                        }

                        if !valid {
                            break;
                        }
                    }

                    if valid {
                        winning = candidate_subset.len();

                        let mut strings = vec![];
                        for c in candidate_subset.iter() {
                            let string = format!("{}{}", c.0, c.1);
                            strings.push(string);
                        }
                        strings.sort();
                        println!("{}: {:?}", candidate_subset.len(), strings.join(","));
                    }
                }
            }
        }

        // And we're done!
        (triples.len() as i32, 0)
    }
}
