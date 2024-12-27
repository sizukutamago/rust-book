use std::io::stdin;

fn main() {
    let mut memory = Memory { slots: vec![] };
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // メモリへの書き込み
        // if tokens[0] == "mem+" {
        //     add_and_print_memory(&mut memory, prev_result);
        //     continue;
        // } else if tokens[0] == "mem-" {
        //     add_and_print_memory(&mut memory, -prev_result);
        //     continue;
        // }

        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            add_and_print_memory(&mut memory, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(&mut memory, tokens[0], -prev_result);
            continue;
        }

        // 式の計算
        let left = eval_token(tokens[0], &memory);
        let right = eval_token(tokens[2], &memory);
        let result = eval_expression(left, tokens[1], right);

        print_output(result);

        prev_result = result;
    }
}

struct Memory {
    slots: Vec<(String, f64)>,
}

fn print_output(value: f64) {
    println!("{}", value);
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
    if token.starts_with("mem") {
        let slot_name = &token[3..];
        // 全てのメモリを探索 (slot.0 が名前(String), slot.1 が値(f64))
        for slot in &memory.slots {
            if slot.0 == slot_name {
                // メモリが見つかったので値を返して終了
                return slot.1;
            }
        }
        // メモリが見つからなかったので初期値を返す
        0.0
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ =>
        // 入力が正しいならここには来ない
        {
            unreachable!()
        }
    }
}

fn add_and_print_memory(memory: &mut Memory, token: &str, prev_result: f64) {
    let slot_name = &token[3..token.len() - 1];

    for slot in memory.slots.iter_mut() {
        if slot.0 == slot_name {
            // メモリが見つかったので、値を更新・表示して終了
            slot.1 += prev_result;
            print_output(slot.1);
            return;
        }
    }

    // メモリが見つからなかったので最後の要素に追加する
    memory.slots.push((slot_name.to_string(), prev_result));
    print_output(prev_result);
}
