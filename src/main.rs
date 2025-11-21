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
use util::system;

use crate::util::io::show_purchased_lotto_amount;

fn main() {
    // 실행 환경 정보 및 최대 구매 금액 출력
    let (os, total_memory, available_memory, max_purchase_amount) = system::get_memories_by_os();
    io::show_memory_information((os, total_memory, available_memory, max_purchase_amount));

    let mut money = input_purchase_amount();

    // 최댓값 입력 편의성 지원
    if money == 0 {
        money = max_purchase_amount;
    }

    let lotto_amount = money / 1000;

    // 처리 스레드 개수 입력
    let threads = input_thread_amount();

    // 처리 시간 측정
    let start = Instant::now();
    let lottos = Lotto::generate_random_lottos(lotto_amount, threads);
    let duration = start.elapsed();
    io::show_duration(duration);

    // 구매 개수 출력
    show_purchased_lotto_amount(&lottos);

    // 당첨 로또 입력
    let winning_lotto = input_winning_lotto();

    // 결과 집계 및 출력
    let mut result: HashMap<Prize, i64> = HashMap::new();
    for lotto in lottos {
        let (match_count, is_bonus_correct) = winning_lotto.get_result(lotto);

        let prize = Prize::get_prize(match_count, is_bonus_correct);

        let count = result.entry(prize).or_insert(0);
        *count += 1
    }
    io::show_result(&result);
    io::show_profit_rate(&result, money as f64);
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
fn input_thread_amount() -> u32 {
    loop {
        let result = io::get_thread_count();
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
