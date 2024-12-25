use itertools::Itertools;
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub struct Day24;

// enum Operation {
//     AND,
//     OR,
//     XOR,
// }

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Gate {
    input1: String,
    input2: String,
    output: String,
    operation: String,
}

impl Gate {
    fn get_output_value(&self, inputs: &HashMap<String, u64>) -> Option<u64> {
        if !inputs.contains_key(&self.input1) || !inputs.contains_key(&self.input2) {
            return None;
        }

        let input1 = inputs.get(&self.input1).unwrap();
        let input2 = inputs.get(&self.input2).unwrap();

        match self.operation.as_str() {
            "AND" => Some(input1 & input2),
            "OR" => Some(input1 | input2),
            "XOR" => Some(input1 ^ input2),
            _ => panic!("Unknown operation"),
        }
    }
}

impl aoc24::DayInner<Day24, u64> for Day24 {
    fn day(&self) -> i32 {
        24
    }

    fn inner(&self, input: String) -> (u64, u64) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let mut part1_inputs_with_values = HashMap::<String, u64>::new();
        let mut og_gates = Vec::<Gate>::new();

        // Parse the input
        let mut in_gates = false;
        for line in lines {
            if line.is_empty() {
                in_gates = true;
                continue;
            }

            if in_gates {
                // Parse the gates
                let parts = line.split(" ").collect::<Vec<&str>>();
                let gate = Gate {
                    input1: parts[0].to_string(),
                    input2: parts[2].to_string(),
                    output: parts[4].to_string(),
                    operation: parts[1].to_string(),
                };
                og_gates.push(gate);
            } else {
                // Parse the inputs
                let parts = line
                    .split(": ")
                    .map(str::to_string)
                    .collect::<Vec<String>>();
                let input = parts[0].clone();
                let value = parts[1].parse::<u64>().unwrap();
                part1_inputs_with_values.insert(input, value);
            }
        }

        run_gates_with_inputs(&mut part1_inputs_with_values, &og_gates);
        let part1 = get_int_value(&part1_inputs_with_values, "z");

        let mut p2_gates = og_gates.clone();
        if !check_no_loop(&p2_gates) {
            panic!("Loop detected");
        }

        swap(&mut p2_gates, "z12", "vdc");
        swap(&mut p2_gates, "nhn", "z21");
        swap(&mut p2_gates, "tvb", "khg");
        swap(&mut p2_gates, "z33", "gst");
        // swap(&mut p2_gates, "nhn", "cdc");
        // swap(&mut p2_gates, "khg", "tbn");
        // swap(&mut p2_gates, "kmm", "z33");

        // println!("Check sum? {:?}", check_sum(&og_gates, 13, 31));
        check_validity(p2_gates.clone(), 45, true);

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let mut target = rng.gen::<u64>();
            println!("Checking target: {}", target);
            while target > 35184372088831 {
                target -= 35184372088831;
            }
            let x = 10000000;
            let y = target - x;

            check_sum(&p2_gates, x, y);
        }

        let mut foo = vec!["z12", "vdc", "nhn", "z21", "tvb", "khg", "z33", "gst"];
        // cdc,khg,kmm,nhn,tbn,vdc,z12,z33
        foo.sort();
        println!("{}", foo.iter().join(","));
        // And we're done!
        (part1, 0)
    }
}

fn swap(gates: &mut Vec<Gate>, output: &str, new_output: &str) {
    let clone = gates.clone();
    for (ii, gate) in clone.iter().enumerate() {
        if gate.output == output {
            gates[ii].output = new_output.to_string();
        }
        if gate.output == new_output {
            gates[ii].output = output.to_string();
        }
    }
}

fn check_validity(mut og_gates: Vec<Gate>, check_depth: usize, _recurse: bool) -> bool {
    let mut locked_set: HashSet<String> = HashSet::new();
    let mut switch_pairs = HashSet::<(String, String)>::new();

    let mut all_outputs = HashSet::<String>::new();
    for gate in og_gates.iter() {
        all_outputs.insert(gate.output.clone());
    }

    for ii in 0..check_depth {
        let z_str = format!("z{:02}", ii);
        println!("Checking: {}", z_str);
        let z_deps = get_dependencies(&og_gates, &z_str);
        println!("{}: {:?}", z_str, z_deps);

        // Is the dependencies a subset of xs and ys of lower or equal value?
        let mut valid_dep_set: HashSet<String> = HashSet::new();
        for jj in 0..ii + 1 {
            let x_str = format!("x{:02}", jj);
            let y_str = format!("y{:02}", jj);
            valid_dep_set.insert(x_str);
            valid_dep_set.insert(y_str);
        }

        // let mut valid = true;

        if valid_dep_set == z_deps.0
            && check_sum(
                &og_gates,
                2_u64.pow(ii as u32) - 1,
                2_u64.pow(ii as u32) - 1,
            ) && check_sum(
                &og_gates,
                2_u64.pow(ii as u32) - 1,
                2_u64.pow(ii as u32),
            )
        {
            println!("Set-wise and sum valid: {}", ii);
            for dep in z_deps.1.iter() {
                if !locked_set.contains(dep) {
                    locked_set.insert(dep.clone());
                }
            }
            locked_set.insert(z_str.clone());
            println!("Locked set now: {:?}", locked_set);

            // for xx in 2_u64.pow(ii as u32)..2_u64.pow((ii + 1) as u32) {
            //     for yy in 2_u64.pow(ii  as u32)..2_u64.pow((ii + 1) as u32) {
            //         if check_sum(&og_gates, xx, yy) {
            //             // println!("Valid: {} + {} = {}", xx, yy, xx + yy);
            //         } else {
            //             println!("Invalid: {} + {} = {}", xx, yy, xx + yy);
            //             valid = false;
            //         }
            //     }
            // }
            // if !check_sum(&og_gates, 2_u64.pow((ii + 1) as u32) -1, 2_u64.pow((ii + 1) as u32) -1) {
            //     println!("Invalid sum");
            //     valid = false;

            //     // What are the new dependencies?
            //     let new_deps = get_dependencies(&og_gates, &z_str);
            //     let old_deps = get_dependencies(&og_gates, &format!("z{:02}", ii-1));

            //     println!("Difference between sets is {:?}", new_deps.1.difference(&old_deps.1));

            //     // Are there any other outputs which are only dependent on valid dep set?
            //     for g in og_gates.iter() {
            //         if !old_deps.1.contains(&g.output) && get_dependencies(&og_gates, &g.output).1.is_subset(&valid_dep_set) {
            //             println!("Candidate: {}", g.output);
            //         }
            //     }
            // }
        } else {
            println!("Invalid: {}", ii);

            // let old_deps = get_dependencies(&og_gates, &format!("z{:02}", ii-1));
            // let new_deps = get_dependencies(&og_gates, &z_str);

            let swap_candidates = all_outputs.difference(&locked_set);

            println!("Swap candidates: {:?}", swap_candidates);

            let mut found_swap = false;

            for pairs in swap_candidates.combinations(2) {
                println!(
                    "[{},{:?}] Trying switch {}<->{}...",
                    ii, switch_pairs, pairs[0], pairs[1]
                );
                swap(&mut og_gates, &pairs[0], &pairs[1]);
                // println!("Checking validity...");

                if !check_no_loop(&og_gates) {
                    // println!("Loop detected - skip");
                    swap(&mut og_gates, &pairs[0], &pairs[1]);
                    continue;
                }

                // println!("Checking sum...");

                let new_z_deps = get_dependencies(&og_gates, &z_str);

                if new_z_deps.2 {
                    // println!("Loop detected - skip");
                    swap(&mut og_gates, &pairs[0], &pairs[1]);
                    continue;
                }

                // println!("Checking if it's a valid swap...");

                if valid_dep_set == new_z_deps.0
                    && check_sum(
                        &og_gates,
                        2_u64.pow(ii as u32) - 1,
                        2_u64.pow(ii as u32) - 1,
                    )
                    && check_sum(
                        &og_gates,
                        2_u64.pow(ii as u32),
                        2_u64.pow(ii as u32) - 1,
                    )
                {
                    println!("Switch {}<->{} worked!", pairs[0], pairs[1]);
                    switch_pairs.insert((pairs[0].clone(), pairs[1].clone()));
                    for dep in valid_dep_set.iter() {
                        if !locked_set.contains(dep) {
                            locked_set.insert(dep.clone());
                        }
                    }
                    locked_set.insert(z_str.clone());
                    println!("Locked set now: {:?}", locked_set);
                    found_swap = true;
                    break;
                } else {
                    // Swap back
                    // println!("Switch {}<->{} failed", pairs[0], pairs[1]);
                    swap(&mut og_gates, &pairs[0], &pairs[1]);
                }
            }

            if !found_swap {
                panic!("No swap found for {}", ii);
            }

            // Let's look for some other candidates...
            // for g in og_gates.iter() {
            //     if !z_deps.1.contains(&g.output) && g.output != z_str {
            //         if recurse {
            //             println!("Swapping: {} -> {}", g.output, z_str);
            //             // Try a swap
            //             let mut new_gates = og_gates.clone();
            //             for (ii, gg) in og_gates.iter().enumerate() {
            //                 if gg.output == g.output {
            //                     new_gates[ii].output = z_str.clone();
            //                 }
            //                 if gg.output == z_str {
            //                     new_gates[ii].output = g.output.clone();
            //                 }
            //             }
            //             let r_valid = check_validity(new_gates, 13, false);
            //             if r_valid {
            //                 return true;
            //             }
            //         }
            //     }
            // }

            // return false;
        }

        // if !valid {
        //     return false;
        // }
        // true
    }

    return true;
}

fn check_valid(inputs_with_values: &HashMap<String, u64>) -> Option<usize> {
    let x = get_int_value(inputs_with_values, "x");
    let y = get_int_value(inputs_with_values, "y");
    let z = get_int_value(inputs_with_values, "z");

    let diff = (x + y) ^ z;

    // println!("{} + {} = {} vs. {} - diff {}", x, y, x + y, z, diff);

    for ii in 0..45 {
        let val = (diff >> ii) & 1;
        if val == 1 {
            return Some(ii);
        }
    }
    return None;
}

fn check_wiring(inputs_with_values: &HashMap<String, u64>) -> Option<usize> {
    let x = get_int_value(inputs_with_values, "x");
    let y = get_int_value(inputs_with_values, "y");
    let z = get_int_value(inputs_with_values, "z");

    let diff = (x + y) ^ z;

    println!("{} + {} = {} vs. {} - diff {}", x, y, x + y, z, diff);

    for ii in 0..45 {
        let val = (diff >> ii) & 1;
        if val == 1 {
            return Some(ii);
        }
    }
    return None;
}

fn check_no_loop(gates: &Vec<Gate>) -> bool {
    for gate in gates.iter() {
        // if gate.input1 == gate.output || gate.input2 == gate.output {
        //     println!("Solo loop detected: {:?}", gate);
        //     return false;
        // }

        // // println!("Checking from gate {}", gate.output);
        // let mut seen_gates = HashSet::<Gate>::new();
        // seen_gates.insert(gate.clone());

        // let mut open_set = Vec::<Gate>::new();
        // for g in gates.iter() {
        //     if g.output == gate.input1 || g.output == gate.input2 {
        //         open_set.push(g.clone());
        //         // open_set.push(g.input2.clone());
        //     }
        // }

        // while !open_set.is_empty() {
        //     let next = open_set.pop().unwrap();
        //     if next == *gate {
        //         println!("Loop detected: {:?}", next);
        //         return false;
        //     }
        //     seen_gates.insert(next.clone());

        //     for g in gates.iter() {
        //         if g.output == next.input1 || g.output == next.input2 {
        //             if seen_gates.contains(&g) || open_set.contains(&g) {
        //                 continue;
        //             }
        //             open_set.push(g.clone());
        //             // open_set.push(g.input2.clone());
        //         }
        //     }
        // }
        if get_parents(gates, gate.output.clone(), HashSet::new()).1 {
            return false;
        }
    }

    true
}

fn get_parents(
    gates: &Vec<Gate>,
    output: String,
    mut children: HashSet<Gate>,
) -> (HashSet<String>, bool) {
    let mut parents = HashSet::<String>::new();
    for gate in gates.iter() {
        if gate.output == output {
            if children.contains(gate) {
                return (HashSet::new(), true);
            }

            children.insert(gate.clone());

            let (new_parents, loop_detected) =
                get_parents(gates, gate.input1.clone(), children.clone());
            if loop_detected {
                return (HashSet::new(), true);
            }
            parents.extend(new_parents);

            let (new_parents, loop_detected) =
                get_parents(gates, gate.input2.clone(), children.clone());
            if loop_detected {
                return (HashSet::new(), true);
            }
            parents.extend(new_parents);

            parents.insert(gate.input1.clone());
        }
    }
    (parents, false)
}

fn get_dependencies(gates: &Vec<Gate>, output: &str) -> (HashSet<String>, HashSet<String>, bool) {
    let mut closed_outputs = HashSet::<String>::new();
    let mut open_outputs = Vec::<String>::new();
    let mut deps = HashSet::<String>::new();
    let mut top_deps = HashSet::<String>::new();

    open_outputs.push(output.to_string());

    while !open_outputs.is_empty() {
        let next = open_outputs.pop().unwrap();
        closed_outputs.insert(next.clone());

        if !next.starts_with("z") {
            deps.insert(next.clone());
        }
        for gate in gates.iter() {
            if gate.output == next {
                // if closed_outputs.contains(&gate.input1) || closed_outputs.contains(&gate.input2) {
                //     return (HashSet::new(), HashSet::new(), true);
                // }
                if !open_outputs.contains(&gate.input1) {
                    open_outputs.push(gate.input1.clone());
                }
                if !open_outputs.contains(&gate.input2) {
                    open_outputs.push(gate.input2.clone());
                }
            }
        }
    }

    for dep in deps.iter() {
        if dep.starts_with("x") || dep.starts_with("y") {
            top_deps.insert(dep.clone());
        }
    }

    (top_deps, deps, false)
}

fn get_int_value(inputs_with_values: &HashMap<String, u64>, prefix: &str) -> u64 {
    let mut val: u64 = 0;
    for (key, value) in inputs_with_values.iter() {
        if key.starts_with(prefix) {
            let z_val = key[1..].parse::<u64>().unwrap();
            val += *value * 2_u64.pow(z_val as u32);
        }
    }
    val
}

fn check_sum(gates: &Vec<Gate>, x: u64, y: u64) -> bool {
    let mut inputs_with_values = HashMap::<String, u64>::new();
    add_inputs("x", x, &mut inputs_with_values);
    add_inputs("y", y, &mut inputs_with_values);

    run_gates_with_inputs(&mut inputs_with_values, &gates);

    let diff = check_valid(&inputs_with_values);

    if diff.is_some() {
        println!("{} + {} != {} vs. {}", x, y, x + y, diff.unwrap());
    } else {
        println!("{} + {} == {}", x, y, x + y);
    }

    return diff.is_none();
}

fn add_inputs(prefix: &str, value: u64, inputs_with_values: &mut HashMap<String, u64>) {
    for ii in 0..45 {
        let key = format!("{}{:02}", prefix, ii);
        let val = (value >> ii) & 1;
        inputs_with_values.insert(key, val);
    }
}

fn run_gates_with_inputs(inputs_with_values: &mut HashMap<String, u64>, gates: &Vec<Gate>) {
    // println!("Running");
    // println!("{:?}", inputs_with_values);
    // println!("{:?}", gates);
    loop {
        let mut found = false;
        // println!("Running with {}", gates.len());
        for gate in gates.iter() {
            if inputs_with_values.contains_key(&gate.output) {
                continue;
            }

            // println!("Checking: {}.", gate.output);

            if let Some(value) = gate.get_output_value(&*inputs_with_values) {
                // println!("Setting {} to {}", gate.output, value);
                found = true;
                inputs_with_values.insert(gate.output.clone(), value);
            }
        }

        if !found {
            break;
        }
    }
    // println!("Done");
}
