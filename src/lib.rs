pub fn return_digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .into_iter()
        .map(|n| n.to_digit(10).unwrap())
        .collect()
}

pub fn number_self_digit_sum(generator: u32) -> u32 {
    generator + sum_of_digits(generator)
}

fn sum_of_digits(number: u32) -> u32 {
    return_digits(number).iter().sum()
}

pub fn is_selfnumber(number: u32) -> bool {
    match number {
        1 => true,
        _ => determine_if_selfnumber(number)
    }
}

fn determine_if_selfnumber(number: u32) -> bool {
    let digits = return_digits(number);
    let generator = match digits.len() {
        1 ... 2 => 1,
        _ => number - (9 * digits.len() as u32)
    };

    (generator..number)
        .into_iter()
        .all(|candidate| number_self_digit_sum(candidate) != number)
}

pub fn list_of_self_numbers(limit: u32) -> Vec<u32> {
    (1..limit+1)
        .into_iter()
        .filter(|n| is_selfnumber(*n))
        .collect()
}
