fn get_no_of_digits(mut n: u32) -> u32 {
    let mut num = 0;
    while n != 0 {
        n /= 10;
        num += 1;
    }
    num
}

pub fn is_armstrong_number(num: u32) -> bool {
    let mut number = num;
    let mut sum = 0;
    let no_of_digits = get_no_of_digits(num);

    while number != 0 {
        let digit = number % 10;
        sum += digit.pow(no_of_digits);
        number /= 10;
    }
    sum == num
}
