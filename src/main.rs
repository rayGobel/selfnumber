extern crate selfnumbers;

fn main() {
    println!("SELF NUMBERS FROM 1-5000");
    let numbers = selfnumbers::list_of_self_numbers(5000);
    println!(" we have {} numbers", numbers.len());
    println!(" with sum of {}", numbers.iter().fold(0, |sum, x| sum + x));
}
