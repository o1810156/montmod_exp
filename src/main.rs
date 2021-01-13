use montmod::*;

fn main() {
    let x = 0xf3d72f88;
    let y = 0xf6c6ba98;
    let n = 0xf981892b;

    println!("res: {:x}", multi5(n, x, y).unwrap());
    println!("res: {:x}", multi7(n, x, y).unwrap());
}