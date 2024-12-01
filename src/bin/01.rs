advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let first_number = split.next().unwrap().parse::<u32>().unwrap();
        let second_number = split.next().unwrap().parse::<u32>().unwrap();
        list1.push(first_number);
        list2.push(second_number);
    }
    list1.sort();
    list2.sort();

    (list1, list2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (list1, list2) = parse_lists(input);

    let mut sum = 0;

    for (pos, _element) in list1.iter().enumerate(){
        sum += list1[pos].abs_diff(list2[pos]);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse_lists(input);
    
    let mut sum = 0;

    for element in list1.iter() {
        sum += *element as usize * list2.iter().filter(|&x| x == element).count();
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
