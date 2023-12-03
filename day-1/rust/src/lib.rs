#[cfg(test)]
mod tests {
    use super::*;
    fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(path)?.parse()?)
    }
    #[test]
    fn sample() -> Result<(), Box<dyn std::error::Error>> {
        let sample_text = reader("../sample.txt")?;
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
        println!("{digits}");
        assert_eq!(digits, 142);

        Ok(())
    }
}