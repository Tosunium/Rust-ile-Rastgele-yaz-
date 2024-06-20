use rand::Rng;
use std::io::{self, Write};
fn main()-> io::Result<()>{

let mut b = rand::thread_rng();
let mut a = rand::thread_rng();



let length = 10; 
let charset = b"abcdefghijklmnopqrstuvwxyz.?#&%()/=)(/&%"; 
let mut rng = rand::thread_rng();
let mut obama = -1;

while obama < 10 {
    let  ab:i64 = b.gen_range(0..100000000000);
let  ba:i64 = a.gen_range(0..100000000000);

let abb:i64 = b.gen_range(0..ab);
let bba:i64 = a.gen_range(0..ba);

let c = abb ^ bba;

    let mut dbe = 1;
    let oba = format!("oba{}", dbe);

let oba: String = (0..length)
    .map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
    })
    .collect();

print!("{}{}",c,oba);

obama+= 1;
dbe+= 1;

}



Ok(())
}
