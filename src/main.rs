use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");

    let secreat_number = rand::thread_rng().gen_range(1, 101);
    println!("shuzi{}",secreat_number);

    println!("haha");


    loop {
        
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法执行");

    let guess:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };

    println!("你的数字：{}", guess);


    match guess.cmp(&secreat_number) {
        Ordering::Less=> println!("Too small"),
        Ordering::Greater=> println!("Too big"),
        Ordering::Equal=> {println!("you win");break;},

    }
}

}
