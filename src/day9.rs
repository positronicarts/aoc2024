pub struct Day9;

#[derive(Clone)]
struct Disc {
    data: Vec<Option<u64>>,
}

impl std::fmt::Debug for Disc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for dd in &self.data {
            match dd {
                Some(d) => write!(f, "{}", d),
                None => write!(f, "."),
            }?;
        }
        Ok(())
    }
}

impl Disc {
    fn checksum(&self) -> u64 {
        let mut sum = 0;
        for (ii, dd) in self.data.iter().enumerate() {
            if let Some(datum) = dd {
                sum += ii as u64 * datum;
            }
        }
        sum as u64
    }
}

impl aoc24::DayInner<Day9, u64> for Day9 {
    fn day(&self) -> i32 {
        9
    }

    fn inner(&self, input: String) -> (u64, u64) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        // let _lines: Vec<&str> = input.lines().collect();
        let data: Vec<u32> = input.chars().map(|l| l.to_string().parse::<u32>().unwrap()).collect();
        let mut disc: Disc = Disc { data: Vec::new() };

        println!("{:?}", data.len());

        let mut gap = false;
        let mut id = 0;
        for datum in data.iter() {
            if gap {
                for _ in 0..*datum {
                    disc.data.push(None);
                }
            } else {
                for _ in 0..*datum {
                    disc.data.push(Some(id));
                }
                id += 1;
            }

            gap = !gap;
        }

        // Part 1
        let mut p1_disc = disc.clone();
        let len = p1_disc.data.len();
        let mut gap_index = 0;

        for ii in 0..p1_disc.data.len() {
            if let Some(d) = p1_disc.data[len-1-ii] {
                for jj in gap_index..len-1-ii    {
                    if p1_disc.data[jj].is_none() {
                        p1_disc.data[jj] = Some(d);
                        p1_disc.data[len-1-ii] = None;
                        gap_index = jj;
                        break;
                    }
                }
            }
        }

        // Part 2
        let mut last_d = -1;

        for ii in 0..disc.data.len() {
            
            if let Some(d) = disc.data[len-1-ii] {
                // How my d's in a row?
                // println!("Checking val {}", d);
                if d as i32 == last_d {
                    continue;
                }
                last_d = d as i32;
                let mut data_len = 0;
                for cc in 1..len-1-ii {
                    if disc.data[len-1-ii-cc] != Some(d) {
                        data_len = cc;
                        break;
                    }
                }

                // println!("Data len {}", data_len);

                // Look for gap
                for jj in 0..len-1-ii    {
                    let mut gap_len = 0;
                    if disc.data[jj].is_none() {
                        // println!("Gap from {}", jj);
                        for cc in 1..len-1-ii {
                            if disc.data[jj+cc].is_some() {
                                gap_len = cc;
                                // println!("Gap len {}", gap_len);
                                break;
                            }
                        }
                    }

                    if gap_len >= data_len {
                        // Move!
                        for cc in 0..data_len {
                            disc.data[len-1-ii-cc] = None;
                            disc.data[jj+cc] = Some(d);
                        }
                        break;
                    }
                }
            }
            // println!("{:?}", disc);
        }


        // And we're done!
        (p1_disc.checksum(), disc.checksum())
    }
}
