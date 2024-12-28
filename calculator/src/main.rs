use std::collections::{hash_map::Entry, HashMap};
use std::io::stdin;

fn main() {
    let mut memory = Memory::new();
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // トークン列に分割
        let tokens = Token::split(&line);

        // 式の評価
        match &tokens[0] {
            Token::MemoryPlus(memory_name) => {
                // メモリへの加算
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, prev_result);
                print_output(result);
            }
            Token::MemoryMinus(memory_name) => {
                // メモリへの減算
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, -prev_result);
                print_output(result);
            }
            _ => {
                // 式の値の計算
                let result = eval_expression(&tokens, &memory);

                print_output(result);
                prev_result = result;
            }
        }
    }

    struct Memory {
        slots: HashMap<String, f64>,
    }

    impl Memory {
        fn new() -> Self {
            Self {
                slots: HashMap::new(),
            }
        }

        fn add(&mut self, slot_name: String, prev_result: f64) -> f64 {
            match self.slots.entry(slot_name) {
                Entry::Occupied(mut entry) => {
                    // メモリが見つかったので値を更新
                    *entry.get_mut() += prev_result;
                    *entry.get()
                }
                Entry::Vacant(entry) => {
                    // メモリが見つからないので要素追加
                    entry.insert(prev_result);
                    prev_result
                }
            }
        }

        fn get(&self, slot_name: &str) -> f64 {
            self.slots.get(slot_name).copied().unwrap_or(0.0)
        }
    }

    #[derive(Debug, PartialEq)]
    enum Token {
        Number(f64),
        MemoryRef(String),
        MemoryPlus(String),
        MemoryMinus(String),
        Plus,
        Minus,
        Asterisk,
        Slash,
    }

    impl Token {
        fn parse(value: &str) -> Self {
            match value {
                "+" => Self::Plus,
                "-" => Self::Minus,
                "*" => Self::Asterisk,
                "/" => Self::Slash,
                _ if value.starts_with("mem") => {
                    let mut memory_name = value[3..].to_string();
                    if value.ends_with('+') {
                        memory_name.pop();
                        Self::MemoryPlus(memory_name)
                    } else if value.ends_with('-') {
                        memory_name.pop();
                        Self::MemoryMinus(memory_name)
                    } else {
                        Self::MemoryRef(memory_name)
                    }
                }
                _ => Self::Number(value.parse().unwrap()),
            }
        }

        fn split(text: &str) -> Vec<Self> {
            text.split(char::is_whitespace).map(Self::parse).collect()
        }
    }

    fn eval_token(token: &Token, memory: &Memory) -> f64 {
        match token {
            Token::Number(value) => {
                // 数値の場合はそのまま返す
                *value
            }
            Token::MemoryRef(memory_name) => {
                // メモリ参照の場合はメモリから値を取得
                memory.get(memory_name)
            }
            _ => {
                // それ以外の場合はエラー
                unreachable!()
            }
        }
    }

    fn eval_expression(tokens: &[Token], memory: &Memory) -> f64 {
        eval_additive(tokens, memory)
    }

    fn eval_additive(tokens: &[Token], memory: &Memory) -> f64 {
        let mut index = 0;
        let mut result;

        (result, index) = eval_multiplicative_expression(tokens, index, memory);

        while index < tokens.len() {
            match &tokens[index] {
                Token::Plus => {
                    let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);

                    result += value;
                    index = next;
                }
                Token::Minus => {
                    let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);
                    result -= value;
                    index = next;
                }
                _ => break,
            }
        }
        result
    }

    fn eval_multiplicative_expression(
        tokens: &[Token],
        index: usize,
        memory: &Memory,
    ) -> (f64, usize) {
        let mut index = index;
        let mut result = eval_token(&tokens[index], memory);
        index += 1;

        while index < tokens.len() {
            match &tokens[index] {
                Token::Asterisk => {
                    result *= eval_token(&tokens[index + 1], memory);
                    index += 2;
                }
                Token::Slash => {
                    result /= eval_token(&tokens[index + 1], memory);
                    index += 2;
                }
                _ => break,
            }
        }
        (result, index)
    }
}

fn print_output(value: f64) {
    println!("{}", value);
}
