use std::collections::HashMap;

pub struct Day24;

// enum Operation {
//     AND,
//     OR,
//     XOR,
// }

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

        let mut inputs_with_values = HashMap::<String, u64>::new();
        let mut gates = Vec::<Gate>::new();

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
                gates.push(gate);
            } else {
                // Parse the inputs
                let parts = line
                    .split(": ")
                    .map(str::to_string)
                    .collect::<Vec<String>>();
                let input = parts[0].clone();
                let value = parts[1].parse::<u64>().unwrap();
                inputs_with_values.insert(input, value);
            }
        }

        loop {
            let mut found = false;
            for gate in gates.iter() {
                if inputs_with_values.contains_key(&gate.output) {
                    continue;
                }

                found = true;

                if let Some(value) = gate.get_output_value(&inputs_with_values) {
                    inputs_with_values.insert(gate.output.clone(), value);
                }
            }

            if !found {
                break;
            }
        }

        // Get the answer
        let mut part1: u64 = 0;

        for (key, value) in inputs_with_values.iter() {
            if key.starts_with("z") {
                let z_val = key[1..].parse::<u64>().unwrap();
                part1 += *value * 2_u64.pow(z_val as u32);
            }
        }

        // And we're done!
        (part1 as u64, 0)
    }
}
