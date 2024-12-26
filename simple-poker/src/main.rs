use std::io;

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deckを作成
    for suit in suits {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }

    // シャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    println!("{:?}", deck);

    // 手札用のVec
    let mut hand: Vec<Card> = Vec::new();

    // 手札にカードを5枚追加
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札を表示
    println!("---Hand---");

    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    println!("入れ替えたいカードの番号を入力してください。(例: 1 3 5)");

    // 手札交換
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    // 扱いやすいようにVecに変換
    let numbers: Vec<usize> = input
        .split_whitespace() //文字列を空白区切りで分割する
        .map(|x| x.parse().unwrap()) //文字列を数値に変換
        .collect::<Vec<usize>>(); //Vecに変換

    // 与えられた数字の箇所をデッキから取り出したカードに置き換える
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札を表示
    println!("---Hand---");

    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }

    // フラッシュの判定
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|card| card.suit == suit); // 1枚目を取得して、全部同じsuitか確認する
                                                           // ペア数のチェック
    let mut count = 0;

    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("フラッシュ!");
    } else if count >= 3 {
        println!("スリーカード!");
    } else if count == 2 {
        println!("ツーペア!");
    } else if count == 1 {
        println!("ワンペア!");
    } else {
        println!("役なし...");
    }
}
