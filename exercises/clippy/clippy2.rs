// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let _option = Some(12);
    if let Some(x) = Some(12) {
      res += x;
    }
    println!("{}", res);
}
