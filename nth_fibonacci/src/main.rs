use std::io;

fn main() {
    let result = loop {
      println!("Welcome, enter the nth fibonacci");
      let mut num = String::new();
  
      io::stdin()
          .read_line(&mut num)
          .expect("Failed to read line.");
  
      let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      if num == 0 || num == 1 {
        break num;
      }

      let mut prev = 0;
      let mut current = 1;

      for i in 0..num {
        if i < 2 {
          println!("Skipping because I don't know how else to do this in rust *shrug emoji*");
        } else {
          let temp = current;
          current = prev + current;
          prev = temp;
        }
      }
      break current;
    };

    println!("The fibonacci number is {}", result);
}
