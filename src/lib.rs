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

    if number == 1 { return true; }

    let mut generator = 1;
    let digits_length = return_digits(number).len();
    if digits_length >= 3 {
        generator = number - (9*digits_length as u32);
    }

    loop {
        // Check for candidancy
        let candidate = number_self_digit_sum(generator);
        if candidate == number {
            return false;
        }

        if generator > number {
            break;
        }

        generator += 1;
    }

    return true;

}

pub fn list_of_self_numbers(limit: u32) -> Vec<u32> {
    (1..limit+1)
        .into_iter()
        .filter(|n| is_selfnumber(*n))
        .collect()
}
