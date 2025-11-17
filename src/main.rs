mod lotto;
mod util;

use lotto::lotto::Lotto;
use lotto::prize::Prize;
use lotto::winning::BonusNumber;
use lotto::winning::WinningLotto;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use util::io;

use crate::util::io::show_purchased_lotto_amount;
use crate::util::system;

fn main() {
    let (os, total_memory, available_memory, max_purchase_amount) = system::get_memories_by_os();
    io::show_memory_information((os, total_memory, available_memory, max_purchase_amount));

    let mut money = input_purchase_amount();

    if money == 0 {
        money = max_purchase_amount;
    }

    let lotto_amount = money / 1000;

    // 처리 시간 측정
    let start = Instant::now();
    let lottos = Lotto::generate_random_lottos(lotto_amount);
    let duration = start.elapsed();

    let ms = duration.as_millis();
    let sec = duration.as_secs_f64();
    io::show_duration(ms, sec);

    show_purchased_lotto_amount(&lottos);

    let winning_lotto = input_winning_lotto();

    let mut result: HashMap<Prize, i64> = HashMap::new();
    for lotto in lottos {
        let (match_count, is_bonus_correct) = winning_lotto.get_result(lotto);

        let prize = Prize::get_prize(match_count, is_bonus_correct);

        let count = result.entry(prize).or_insert(0);
        *count += 1
    }

    io::show_result(&result);
    io::show_profit_rate(&result, money as f64);
    thread::sleep(Duration::from_secs(5));
}

fn input_purchase_amount() -> u64 {
    loop {
        let result = io::input_purchase_amount();
        match result {
            Ok(value) => {
                break value;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn input_winning_lotto() -> WinningLotto {
    loop {
        let winning_numbers = loop {
            let result = io::input_winning_lotto();
            match result {
                Ok(value) => {
                    break value;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        };

        let winning_lotto = loop {
            let result = io::input_bonus_lotto();

            match result {
                Ok(value) => {
                    break value;
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        };

        match WinningLotto::new(winning_numbers, winning_lotto) {
            Ok(value) => break value,
            Err(_e) => println!("{}", _e),
        }
    }
}
