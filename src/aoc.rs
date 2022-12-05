pub struct AocYear {
    // days: [Option<Day>; 25],
    // we use a vector here because initializing the rest of an array with
    // default is very hard
    year: u32,
    days: Vec<Day>,
}

impl AocYear {
    pub fn display_days(&self) {
        // Run each day and display results
        for (idx, day) in self.days.iter().enumerate() {
            println!(
                "[year {}, day {:>2}, part: 1] result: {}",
                self.year,
                idx,
                day.run_part1()
            );
            println!(
                "[year {}, day {:>2}, part: 2] result: {}",
                self.year,
                idx,
                day.run_part2()
            );
        }
    }
}

pub struct Day {
    input: &'static str,
    part1: Option<fn(&str) -> String>,
    part2: Option<fn(&str) -> String>,
}

impl Day {
    fn run_part(&self, part: &Option<fn(&str) -> String>) -> String {
        match part {
            Some(x) => x(self.input),
            None => "None".to_string(),
        }
    }

    fn run_part1(&self) -> String {
        self.run_part(&self.part1)
    }

    fn run_part2(&self) -> String {
        self.run_part(&self.part2)
    }
}
