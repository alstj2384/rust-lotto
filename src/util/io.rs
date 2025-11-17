use num_format::Locale;
use num_format::ToFormattedString;

use crate::BonusNumber;
use crate::Lotto;
use crate::lotto::lotto::LOTTO_PRICE;
use crate::lotto::lotto::LOTTO_SIZE;
use crate::lotto::prize::Prize;
use crate::util::system::MAX_PURCHASE_AMOUNT;
use std::collections::HashMap;
use std::io;

const DELIMITER: &str = ",";

pub fn input_purchase_amount() -> Result<u64, String> {
    println!("구매금액을 입력해 주세요.(0을 입력하면 최대 금액으로 구매합니다)");
    let mut input = String::new();
    if let Err(_e) = io::stdin().read_line(&mut input) {
        return Err("[ERROR] 잘못된 입력입니다.".to_string());
    }

    let money: u64 = match input.trim().parse::<u64>() {
        Ok(value) => value,
        Err(_e) => {
            return Err("[ERROR] 구매 금액은 숫자만 입력할 수 있습니다.".to_string());
        }
    };

    if money < 0 || money > MAX_PURCHASE_AMOUNT {
        return Err(format!(
            "[ERROR] 구매 금액은 0원에서 {}원 미만이어야 합니다.",
            MAX_PURCHASE_AMOUNT
        ));
    }

    if money % 1000 != 0 {
        return Err(format!(
            "[ERROR] 구매 금액은 {}원 단위여야 합니다.",
            LOTTO_PRICE
        ));
    }
    println!();
    Ok(money)
}

pub fn input_winning_lotto() -> Result<Lotto, String> {
    println!("당첨 번호를 입력해 주세요.");
    let mut input = String::new();
    if let Err(_e) = io::stdin().read_line(&mut input) {
        return Err("[ERROR] 잘못된 입력입니다.".to_string());
    }

    let parsed = input.split(DELIMITER);

    let mut lotto_numbers = [0i8; LOTTO_SIZE];

    let mut i = 0;
    for number in parsed {
        match number.trim().parse() {
            Ok(value) => lotto_numbers[i] = value,
            Err(_e) => return Err("[ERROR] 로또 번호는 숫자여야 합니다.".to_string()),
        }
        i += 1;
    }

    match Lotto::new(lotto_numbers) {
        Ok(value) => Ok(value),
        Err(_e) => Err(_e),
    }
}

pub fn input_bonus_lotto() -> Result<BonusNumber, String> {
    println!("보너스 번호를 입력해 주세요.");
    let mut input = String::new();
    if let Err(_e) = io::stdin().read_line(&mut input) {
        return Err("[ERROR] 잘못된 입력입니다.".to_string());
    }

    let bonus_number = match input.trim().parse::<i8>() {
        Ok(value) => value,
        Err(_e) => return Err("[ERROR] 보너스 번호는 숫자만 입력해야 합니다.".to_string()),
    };
    println!();
    match BonusNumber::new(bonus_number) {
        Ok(value) => Ok(value),
        Err(_e) => Err(_e),
    }
}

pub fn show_purchased_lotto_amount(lottos: &Vec<Lotto>) {
    println!(
        "{}개를 구매했습니다.",
        lottos.len().to_formatted_string(&Locale::ko)
    );
    println!();
}

pub fn show_result(result: &HashMap<Prize, i64>) {
    println!("당첨 통계");
    println!("---");
    println!(
        "{}개 일치 ({}원) - {}개",
        3,
        "5,000",
        result.get(&Prize::Fifth).unwrap_or(&0)
    );
    println!(
        "{}개 일치 ({}원) - {}개",
        4,
        "50,000",
        result.get(&Prize::Fourth).unwrap_or(&0)
    );
    println!(
        "{}개 일치 ({}원) - {}개",
        5,
        "1,500,000",
        result.get(&Prize::Third).unwrap_or(&0)
    );
    println!(
        "{}개 일치, 보너스 볼 일치 ({}원) - {}개",
        5,
        "30,000,000",
        result.get(&Prize::Second).unwrap_or(&0)
    );
    println!(
        "{}개 일치 ({}원) - {}개",
        6,
        "2,000,000,000",
        result.get(&Prize::First).unwrap_or(&0)
    );
}

pub fn show_profit_rate(result: &HashMap<Prize, i64>, money: f64) {
    let mut sum: i64 = 0;
    for prize in result {
        sum += Prize::get_sum(prize.0, prize.1);
    }

    println!(
        "총 수익률은 {:.1}%입니다.",
        sum as f64 / money as f64 * 100.0
    );
}

pub fn show_memory_information(system_infos: (String, u64, u64, u64)) {
    println!("현재 실행중인 운영체제: {}", system_infos.0);
    println!();
    println!("총 메모리: {}", format_mem(system_infos.1));
    println!("사용 가능한 메모리: {}", format_mem(system_infos.2));
    println!();

    println!(
        "최대 구매 가능 금액: {}원 (사용 가능 메모리 기반)",
        system_infos.3.to_formatted_string(&Locale::ko)
    );
    println!();
}

pub fn show_duration(ms: u128, sec: f64) {
    println!("생성 소요 시간: {} ms ({} s)", ms, sec);
}

fn format_mem(bytes: u64) -> String {
    format!(
        "{} B / {} KB / {} MB",
        bytes,
        bytes / 1024,
        bytes / 1024 / 1024
    )
}
