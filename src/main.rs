mod lotto;
mod util;

use lotto::lotto::Lotto;
use lotto::prize::Prize;
use lotto::winning::BonusNumber;
use lotto::winning::WinningLotto;
use std::collections::HashMap;
use std::fs;
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

// 벡터 내 요소의 크기 합 구하기
fn vec_heap_size_i32(v: &Vec<i32>) -> usize {
    v.capacity() * size_of::<i32>()
}

// Lotto 구조체 자체 크기 + 벡터 내 요소 크기 합 구하기
fn lotto_runtime_size(l: &Lotto) -> usize {
    size_of::<Lotto>() + vec_heap_size_i32(&l.lotto_numbers)
}

// Vec<Lotto> 내부 요소의 전체 크기 구하기
fn lotto_vec_runtime_size(v: &Vec<Lotto>) -> usize {
    v.iter().map(|l| lotto_runtime_size(l)).sum()
}

fn main() {
    // 메모리 측정 코드
    let mem_used = read_value("/sys/fs/cgroup/memory.current");
    let mem_limit = read_value("/sys/fs/cgroup/memory.max");
    let swap_current = read_value("/sys/fs/cgroup/memory.swap.current");
    let swap_limit = read_value("/sys/fs/cgroup/memory.swap.max");

    println!("Memory Used:   {}", format_mem(mem_used));
    println!("Memory Limit:  {}", format_mem(mem_limit));
    println!("Swap Used: {}", format_mem(swap_current));
    println!("Swap Limit: {}", format_mem(swap_limit));

    // 구조체 Struct 크기
    println!("{}", size_of::<Lotto>()); // 24Byte(pointer(8) + length(8) + capacity(8))
    println!("{}", size_of::<WinningLotto>()); // 32Byte()

    let money = input_purchase_amount();

    let lotto_amount = money / 1000;
    let lottos = Lotto::generate_random_lottos(lotto_amount);

    show_purchased_lotto_amount(&lottos);
    show_purchased_lotto(&lottos);

    // 로또 1개 발행 -> Lotto 구조체(24) + Vec(capacity 8 * 4byte => 32byte) = 56
    let used_mem = lotto_vec_runtime_size(&lottos);
    println!(
        "Vec<Lotto> 요소 크기(Byte, KB, MB): ({}, {}, {})",
        used_mem,
        used_mem / 1024,
        used_mem / 1024 / 1024
    );

    // Lotto 1개의 크기: 56 Byte

    // 남은 메모리 대비 생성 가능한 객체 개수:
    let mem_left = match (mem_used, mem_limit) {
        (Some(u), Some(l)) => Some(l.saturating_sub(u)),
        _ => None,
    };

    println!(
        "Memory Left: {}",
        format_mem(match (mem_used, mem_limit) {
            (Some(u), Some(l)) => Some(l.saturating_sub(u)),
            _ => None,
        })
    );

    let mem_used = read_value("/sys/fs/cgroup/memory.current");
    println!("Memory Used:   {}", format_mem(mem_used));

    // Lotto 벡터가 차지한 메모리까지 제외한 남은 공간
    let mem_left_after_lotto = mem_left.map(|l| l.saturating_sub(used_mem as u64));

    println!(
        "Memory Left (after Vec<Lotto>): {}",
        format_mem(mem_left_after_lotto)
    );
    let swap_current = read_value("/sys/fs/cgroup/memory.swap.current");

    println!(
        "Swap Memory current (after Vec<Lotto>: {}",
        format_mem(swap_current)
    );

    let winning_lotto = input_winning_lotto();

    let mut result: HashMap<Prize, i32> = HashMap::new();
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
