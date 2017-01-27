pub fn return_digits(num: u32) -> Vec<u32> {
    let num_str = num.to_string();
    let mut result = vec![];
    for digit in num_str.chars() {
        result.push(digit.to_digit(10).unwrap());
    }
    result

}

pub fn number_self_digit_sum(generator: u32) -> u32 {
    let sum_of_digits = sum_of_digits(generator);
    generator + sum_of_digits
}

fn sum_of_digits(number: u32) -> u32 {
    return_digits(number).iter().fold(0, |sum, x| sum + x)
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
    let mut self_numbers = vec![];

    for candidate in 1..limit+1 {
        if is_selfnumber(candidate) {
            self_numbers.push(candidate);
        }
    }
    return self_numbers;
}
