use std::fs;

mod elf {
    use std::ops::RangeInclusive;

    #[derive(PartialEq)]
    pub(crate) struct ElfSections {
        section_range: RangeInclusive<u32>,
    }

    impl ElfSections {
        // 'Box' is a smart pointer that is used to allocate memory on the heap and store values in it.
        // It provides ownership and ensures that the memory is properly deallocated when it goes out of scope.
        //
        // 'dyn' is a prefex of a trait object's type. 'dyn' because the compiler uses dynamic dispatch
        // to determine the type of returned object at runtime.
        // The compiler guarantees that the any returned object implements the trait 'std::error::Error'.
        pub(crate) fn try_from(string: &str) -> Result<ElfSections, Box<dyn std::error::Error>> {
            let mut string_split = string.split('-');
            let start_section = string_split
                .next()
                .ok_or("Missing section")?
                .parse::<u32>()?;
            let end_section = string_split
                .next()
                .ok_or("Missing section")?
                .parse::<u32>()?;
            let section_range = RangeInclusive::new(start_section, end_section);
            Ok(Self { section_range })
        }

        pub(crate) fn contains(&self, other: &ElfSections) -> bool {
            self.section_range.contains(other.section_range.start())
                && self.section_range.contains(other.section_range.end())
        }

        pub(crate) fn overlaps(&self, other: &ElfSections) -> bool {
            self.section_range.contains(other.section_range.start())
                || other.section_range.contains(self.section_range.start())
        }
    }
}

pub fn part1(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let lines = puzzle_input.split('\n');
    let mut total = 0;
    for line in lines {
        let mut line_split = line.split(',');
        let elf1 = elf::ElfSections::try_from(line_split.next().unwrap()).unwrap();
        let elf2 = elf::ElfSections::try_from(line_split.next().unwrap()).unwrap();
        if elf1.contains(&elf2) || elf2.contains(&elf1) || elf1 == elf2 {
            total += 1;
        }
    }
    total
}

pub fn part2(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let lines = puzzle_input.split('\n');
    let mut total = 0;
    for line in lines {
        let mut line_split = line.split(',');
        let elf1 = elf::ElfSections::try_from(line_split.next().unwrap()).unwrap();
        let elf2 = elf::ElfSections::try_from(line_split.next().unwrap()).unwrap();
        if elf1.overlaps(&elf2) {
            total += 1;
        }
    }
    total
}
