mod parser;

fn get_code_point(point: char) -> u32 {
    return point as u32;
}
fn main() {
    println!("{:x}", get_code_point(' '));
}
