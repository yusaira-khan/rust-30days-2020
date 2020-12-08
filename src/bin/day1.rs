use std::io;

fn main() -> io::Result<()> {
    let i1 = 4;
    let d1 = 4.0;
    let s1: String = "hello! ".to_string();
    let mut buffer1 = String::new();
    let mut buffer2 = String::new();
    let mut buffer3 = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer1)?;
    let i2 = buffer1.trim().parse::<i32>().unwrap();
    stdin.read_line(&mut buffer2)?;
    let d2 = buffer2.trim().parse::<f32>().unwrap();
    stdin.read_line(&mut buffer3)?;
    let s2: String = buffer3;
    let i_s: String = (i1 + i2).to_string();
    let d_s: String = (d1 + d2).to_string();
    let s_c: String = s1 + &s2;
    println!("{}", i_s);
    println!("{}", d_s);
    println!("{}", s_c);
    return Ok(());
}
