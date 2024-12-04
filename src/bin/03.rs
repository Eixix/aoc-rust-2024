use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|e| e.extract()) {
        let num_x = x.parse::<u32>().unwrap();
        let num_y = y.parse::<u32>().unwrap();
        sum += num_x * num_y;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex_builder = "(do^n|don't|".to_owned() + &*input.chars().next().unwrap().to_string() + ")" + r".*?mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(&regex_builder).unwrap();
    let mut sum = 0;
    for (_, [bool, x, y]) in re.captures_iter(input).map(|e| e.extract()) {
        let do_it = bool != "don't";
        let num_x = x.parse::<u32>().unwrap();
        let num_y = y.parse::<u32>().unwrap();


        sum += if do_it { num_x * num_y } else { 0 };
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
