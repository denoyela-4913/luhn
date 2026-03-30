/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    let input_code: String = String::from(code).chars().filter(|c: &char| !c.is_whitespace()).collect::<String>();

    // Debug : print the input code after removing whitespace
    // println!("|{}| => |{}|", code, input_code);

    if input_code.len() <= 1 || !input_code.chars().all(|c: char| c.is_ascii_digit()) {
        // Debug : print the result
        // println!("false");
        return false;
    }
    // Converting string to vector of integers
    let mut vecnb : Vec<u8> = input_code.as_bytes().iter().map(|&b| b - b'0').collect();
    
    // Debug : print the original vector
    // println!("Before - vecnb: {:?}", vecnb);

    for n in vecnb.iter_mut().rev().skip(1).step_by(2) {
        let mut double = *n * 2;
        if double > 9 {
            double -= 9;
        }

        // Debug : print the original number and the doubled value
        // println!("{} => {}", *n, double);

        *n = double;
    };

    // Debug : print the modified vector
    // println!("After  - vecnb: {:?}", vecnb);

    let sum: u32 = vecnb.iter().map(|&n| n as u32).sum();

    // Debug : print the sum
    // println!("sum: {}", sum);

    if ! sum.is_multiple_of(10) {

        // Debug : print the result
        // println!("false");

        return false;
    }
    // Debug : print the result
    // println!("true");

    true
} // fn is_valid


fn main() {
    println!("Hello, world! luhn code validator");
    is_valid(" 4539 3195 0343 6467 ");
    is_valid("0");
    is_valid("   1   ");
    is_valid("125");
}
