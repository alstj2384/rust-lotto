mod lotto;
mod util;

use lotto::lotto::Lotto;
use lotto::prize::Prize;
use lotto::winning::BonusNumber;
use lotto::winning::WinningLotto;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use util::io;

use crate::util::io::show_purchased_lotto;
use crate::util::io::show_purchased_lotto_amount;

fn read_value(path: &str) -> Option<u64> {
    let content = fs::read_to_string(path).ok()?.trim().to_string();
    if content == "max" {
        None // unlimited
    } else {
        content.parse::<u64>().ok()
    }
}

fn format_mem(opt: Option<u64>) -> String {
    match opt {
        Some(bytes) => format!(
            "{} B / {} KB / {} MB",
            bytes,
            bytes / 1024,
            bytes / 1024 / 1024
        ),
        None => "unlimited".to_string(),
    }
}

fn lotto_runtime_size() -> usize {
    size_of::<[i8; 6]>()
}

fn get_max_purchase_amount(free_memory: u64, size: u64, padding: u64, lotto_price: u64) -> u64 {
    free_memory / size * padding / 100 * lotto_price
}

fn main() {
    // 메모리 측정 코드
    let mem_used = read_value("/sys/fs/cgroup/memory.current");
    let mem_limit = read_value("/sys/fs/cgroup/memory.max");

    println!("Memory Used:   {}", format_mem(mem_used));
    println!("Memory Limit:  {}", format_mem(mem_limit));

    let max_bound = match mem_limit {
        Some(value) => get_max_purchase_amount(value, lotto_runtime_size() as u64, 80, 1000),
        None => 0,
    };

    println!("최대 구매 가능 금액: {}", max_bound);
    let money = input_purchase_amount();

    let lotto_amount = money / 1000;

    // 처리 시간 측정
    let start = Instant::now();
    let lottos = Lotto::generate_random_lottos(lotto_amount);

    let duration = start.elapsed();

    show_purchased_lotto_amount(&lottos);
    // show_purchased_lotto(&lottos);

    let ms = duration.as_millis();
    let sec = duration.as_secs_f64();
    println!("생성 소요 시간: {} ms ({} s)", ms, sec);

    println!("spend money: {}원({}개의 인스턴스)", money, money / 1000);
    // 객체 생성 이후 사용한 메모리 공간
    let mem_used = read_value("/sys/fs/cgroup/memory.current");
    println!("Memory Used:   {}", format_mem(mem_used));

    // 객체 생성 이후 남은 메모리 공간
    println!(
        "Memory left:   {}",
        format_mem(Option::Some(mem_limit.unwrap() - mem_used.unwrap()))
    );

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
}

fn input_purchase_amount() -> i64 {
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
