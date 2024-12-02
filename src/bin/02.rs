advent_of_code::solution!(2);

fn check_for_safety(numbers: &Vec<u32>) -> bool {
    let increasing = numbers[0] <= numbers[1];
    match increasing {
        false => numbers.windows(2).all(|x| x[0] == x[1] + 1 || x[0] == x[1] + 2 || x[0] == x[1] + 3),
        true => numbers.windows(2).all(|x| x[0] + 1 == x[1] || x[0] + 2 == x[1] || x[0] + 3 == x[1])
    }
}

fn get_slices_of_length_n<T: Clone>(n: usize, vec: Vec<T>) -> Vec<Vec<T>> {
    fn combinations<T: Clone>(vec: &[T], n: usize, start: usize, current: &mut Vec<T>, result: &mut Vec<Vec<T>>) {
        if current.len() == n {
            result.push(current.clone());
            return;
        }

        for i in start..vec.len() {
            current.push(vec[i].clone());
            combinations(vec, n, i + 1, current, result);
            current.pop();
        }
    }

    let mut result = Vec::new();
    let mut current = Vec::new();
    combinations(&vec, n, 0, &mut current, &mut result);
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;

    for line in input.lines() {
        let numbers = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let is_safe = check_for_safety(&numbers);
        safe_reports += if is_safe { 1 } else { 0 };
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports = 0;

    for line in input.lines() {
        let numbers = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let is_safe = check_for_safety(&numbers) || get_slices_of_length_n(numbers.len()-1, numbers).iter().any(|x| check_for_safety(&x));
        safe_reports += if is_safe { 1 } else { 0 };
    }

    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
