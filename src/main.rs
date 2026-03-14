use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::process::Command;

fn main() {
  let mut x:u64 = 0;
  loop {
    if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {/*将持续时间转换为秒*/x = duration.as_secs();} else {};
    if x % 2 == 0 {
    println!("                                      ");
    println!("                                      ");
    println!("                /\\                   ");
    println!("         /\\     \\/     /\\          ");
    println!("         \\/     ||     \\/           ");
    }else if x % 4 == 1 {
    println!("                                      ");
    println!("                                      ");
    println!("               /\\                    ");
    println!("        /\\      \\/    /\\           ");
    println!("         \\/     ||     \\/           ");
    }else if x % 4 == 3 {
    println!("                                      ");
    println!("                                      ");
    println!("                 /\\                  ");
    println!("          /\\    \\/      /\\         ");
    println!("         \\/     ||     \\/           ");
    }
    println!("         ||     ||     ||             ");
    println!("         ||   __||__   ||             ");
    println!("     __--||~~~  ||, ~~~||--__         ");
    println!("    |__  || ,  .  ,    || ,__|        ");
    println!("    |  ~~~----______----~~~  |        ");
    println!("    |  ,      .     .     ,  |        ");
    println!("    | ,  .       ,    .      |        ");
    println!("    |__        .    ,      __|        ");
    println!("    |  ~~~----______----~~~  |        ");
    println!("    | ,     ,         .   ,  |        ");
    println!("    |    ,     ,         .   |        ");
    println!("    |__   .        .   ,   __|        ");
    println!("       ~~~----______----~~~           ");
    println!("                                      ");
    println!("                                      ");
    println!("生日快乐！");
    println!("祝你在新的一岁里，身体健康，心想事成，万事如意！");
    sleep(Duration::from_millis(100));
    Command::new("cmd").arg("/c").arg("cls").status().expect("failed to execute process");
  }
}