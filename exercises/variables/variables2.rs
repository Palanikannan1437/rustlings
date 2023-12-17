// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let x: &str = "11";
    let x: u32 = x.trim().parse().expect("x is not a number!");
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
