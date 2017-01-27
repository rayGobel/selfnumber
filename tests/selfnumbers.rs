extern crate selfnumbers;

#[test]
fn test_return_digits_10() {
    let number = 10;
    assert_eq!(selfnumbers::return_digits(number), vec![1, 0]);
}

#[test]
fn test_return_digits_19() {
    let number = 19;
    assert_eq!(selfnumbers::return_digits(number), vec![1, 9]);
}

#[test]
fn test_return_digits_3550() {
    let number = 3550;
    assert_eq!(selfnumbers::return_digits(number), vec![3, 5, 5, 0]);
}

#[test]
fn test_number_self_digit_sum() {
    let generator = 35;
    assert_eq!(selfnumbers::number_self_digit_sum(generator), 35 + 3 + 5);
}

#[test]
fn test_number_self_digit_sum_75() {
    let generator = 75;
    assert_eq!(selfnumbers::number_self_digit_sum(generator), 87);
}

#[test]
fn test_if_self_number_21() {
    let selfnumber_candidate = 21;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), false);
}

#[test]
fn test_if_self_number_40() {
    let selfnumber_candidate = 40;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), false);
}

#[test]
fn test_if_self_number_20() {
    let selfnumber_candidate = 20;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), true);
}

#[test]
fn test_if_self_number_2() {
    let selfnumber_candidate = 2;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), false);
}

#[test]
fn test_if_self_number_97() {
    let selfnumber_candidate = 97;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), true);
}

#[test]
fn test_if_self_number_490() {
    let selfnumber_candidate = 490;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), true);
}

#[test]
fn test_if_self_number_525() {
    let selfnumber_candidate = 525;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), true);
}

#[test]
fn test_if_self_number_4999() {
    let selfnumber_candidate = 4999;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), true);
}

#[test]
fn test_if_self_number_11() {
    let selfnumber_candidate = 11;
    assert_eq!(selfnumbers::is_selfnumber(selfnumber_candidate), false);
}

#[test]
fn test_list_of_self_numbers_30() {
    let limit = 30;
    assert_eq!(selfnumbers::list_of_self_numbers(limit), vec![1, 3, 5, 7, 9, 20]);

}

#[test]
fn test_list_of_self_numbers_100() {
    let limit = 100;
    assert_eq!(selfnumbers::list_of_self_numbers(limit), vec![1, 3, 5, 7, 9, 20, 31, 42, 53, 64, 75, 86, 97]);

}
