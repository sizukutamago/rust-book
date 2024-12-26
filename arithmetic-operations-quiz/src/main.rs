use std::i32;

use rand::Rng;

fn main() {
    let mut num_of_correct = 0;

    while num_of_correct < 3 {
        let quiz_mode = rand::thread_rng().gen_range(1..=2);

        match quiz_mode {
            1 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} + {} = ??", op1, op2);
                println!("?? の値を入力してください: ");

                let mut ans_input = String::new();

                std::io::stdin().read_line(&mut ans_input).ok();

                let ans_input = ans_input.trim().parse::<i32>().ok();

                match ans_input {
                    Some(ans_input) => {
                        dbg!(ans_input);
                    }
                    None => {
                        println!("数値を入力してください");
                        continue;
                    }
                }

                if dbg!(ans_input.unwrap() == op1 + op2) {
                    println!("正解です！");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解です！");
                }
            },
            2 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} - {} = ??", op1, op2);
                println!("?? の値を入力してください: ");

                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).ok();

                let ans_input = ans_input.trim().parse::<i32>().ok();
                match ans_input {
                    Some(ans_input) => {
                        dbg!(ans_input);
                    }
                    None => {
                        println!("数値を入力してください");
                        continue;
                    }
                }

                if dbg!(ans_input == Some(op1 - op2)) {
                    println!("正解です！");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解です！");
                }
            },
            _ => unreachable!(),
        }
    }

    println!("クリア！");
    println!("i32が扱えるデータ範囲: {} ~ {}", i32::MIN, i32::MAX);
    println!("u32が扱えるデータ範囲: {} ~ {}", u32::MIN, u32::MAX);
}
