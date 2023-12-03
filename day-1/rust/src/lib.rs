const NUMLIST: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const THREE_LETTER: [usize; 3] = [1, 2, 6];
const FOUR_LETTER: [usize; 3] = [4, 5, 9];
const FIVE_LETTER: [usize; 3] = [3, 7, 8];
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::ops::Deref;
    use std::time::{Duration, Instant};
    use substring::Substring;
    use unicode_segmentation::UnicodeSegmentation;
    fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(path)?.parse()?)
    }
    fn part_1_helper(location: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let sample_text = reader(location)?;
        let sample_list = sample_text
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();

        let digits: u32 = sample_list
            .iter()
            .filter(|&entry| !entry.is_empty())
            .enumerate()
            .map(|(i, raw_entry)| {
                println!("I: {i} ENTRY: {raw_entry}");
                let numbers = raw_entry
                    .chars()
                    .filter_map(|ch| ch.to_digit(10))
                    .collect::<Vec<u32>>();
                // NOTE: primitive clone is cheap, do that here instead of worrying of an extra
                // alloc.
                println!("{numbers:?}");
                let head = numbers.get(0).map(u32::clone).unwrap_or(0);
                let tail = numbers.get(numbers.len() - 1).map(u32::clone).unwrap_or(0);
                head * 10 + tail
            })
            .sum();
        Ok(digits)
    }
    #[test]
    fn sample() -> Result<(), Box<dyn std::error::Error>> {
        let digits = part_1_helper("../sample.txt")?;
        println!("{digits}");
        assert_eq!(digits, 142);

        Ok(())
    }
    #[test]
    fn part_1() -> Result<(), Box<dyn std::error::Error>> {
        let digits = part_1_helper("../input.txt")?;
        println!("{digits}");
        assert_eq!(digits, 53194);

        Ok(())
    }
    #[test]
    fn part2() -> Result<(), Box<dyn std::error::Error>> {
        let sample_text = reader("../input.txt")?;
        let sample_list = sample_text
            .split('\n')
            .map(String::from)
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<String>>();
        let digits: usize = sample_list
            .iter()
            .enumerate()
            .map(|(loc, raw_entry)| {
                println!("LOC: {loc} INITIAL ENTRY: {raw_entry}");
                // TODO: build my own substring.
                // HACK: using chars here where i should be using graphemes
                let mut ret: Vec<usize> = vec![];
                let indices: Vec<(usize, &str)> = raw_entry.graphemes(true).enumerate().collect();
                let len = indices.len() - 1;
                let mut i = 0;
                // TODO: write this as while let loop instead
                // TODO: optimize the multiple incrementors
                'god: loop {
                    if let Some((_, char)) = indices.get(i) {
                        if let Ok(n) = usize::from_str_radix(char, 10) {
                            ret.push(n);
                        } else {
                            if let (Some((start, _)), Some((end, _))) =
                                (indices.get(i), indices.get(i + 2))
                            {
                                let candidate = raw_entry.substring(*start, *end + 1);
                                for index in THREE_LETTER {
                                    if let Some(word) = NUMLIST.get(index - 1) {
                                        if word.deref() == candidate {
                                            ret.push(index);
                                            i += 2;
                                            continue 'god;
                                        }
                                    }
                                }
                            } else {
                                // WARN: if we cannot slice a three letter word then exit the
                                // branch early
                                i += 1;
                                continue 'god;
                            }

                            if let (Some((start, _)), Some((end, _))) =
                                (indices.get(i), indices.get(i + 3))
                            {
                                let candidate = raw_entry.substring(*start, *end + 1);
                                for index in FOUR_LETTER {
                                    if let Some(word) = NUMLIST.get(index - 1) {
                                        if word.deref() == candidate {
                                            ret.push(index);
                                            // booooo! could not optimize this.
                                            // i += 4;
                                            i += 3;
                                            continue 'god;
                                        }
                                    }
                                }
                            }
                            if let (Some((start, _)), Some((end, _))) =
                                (indices.get(i), indices.get(i + 4))
                            {
                                let candidate = raw_entry.substring(*start, *end + 1);
                                for index in FIVE_LETTER {
                                    if let Some(word) = NUMLIST.get(index - 1) {
                                        if word.deref() == candidate {
                                            ret.push(index);
                                            // booooo! could not optimize this.
                                            // i += 5;
                                            i += 4;
                                            continue 'god;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // NOTE: exit the loop if there are no more entries in the vec
                        break;
                    }
                    i += 1;
                }
                println!("{ret:?}");
                let head = ret.first().map(usize::clone).unwrap_or(0);
                let tail = ret.last().map(usize::clone).unwrap_or(0);
                let ret = head * 10 + tail;
                println!("values! {ret}");
                ret
            })
            .sum();
        println!("PART2: SOLUTION: {digits}");
        Ok(())
    }
}
