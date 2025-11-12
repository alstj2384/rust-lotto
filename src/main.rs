mod lotto;

use lotto::lotto::Lotto;
use lotto::prize::Prize;
use lotto::winning::BonusNumber;
use lotto::winning::WinningLotto;
use std::collections::HashMap;
use std::io;

fn main() {
    // 구매할 로또 금액 입력받기
    let money = loop {
        let result = input_purchase_amount();
        match result {
            Ok(value) => {
                break value;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    };
    println!();

    // 로또 생성하기
    let lotto_amount = money / 1000;

    let mut lottos: Vec<Lotto> = Vec::new();
    while lottos.len() != lotto_amount.try_into().unwrap() {
        lottos.push(Lotto::generate_by_random());
    }

    // 생성된 로또 개수 보여주기
    println!("{}개를 구매했습니다.", lottos.len());

    // 생성된 로또 보여주기
    for lotto in &lottos {
        println!("{}", lotto)
    }
    println!();

    let winning_lotto = loop {
        // 당첨 로또 번호 입력받기
        let winning_numbers = loop {
            let result = input_winning_lotto();
            match result {
                Ok(value) => {
                    break value;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        };

        // 보너스 번호 입력받기
        let winning_lotto = loop {
            let result = input_bonus_lotto();

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
    };
    println!();

    // 결과를 출력하기

    let mut result: HashMap<Prize, i32> = HashMap::new();
    // 1. 로또 번호들과 기본 번호 + 당첨 번호를 비교하기
    for lotto in lottos {
        let (match_count, is_bonus_correct) = winning_lotto.get_result(lotto);

        let prize = Prize::get_prize(match_count, is_bonus_correct);

        let count = result.entry(prize).or_insert(0);
        *count += 1
    }

    println!("당첨 통계");
    println!("---");
    // 2. 당첨 등수를 산정하고 결과 Map에 저장하기, 총 수익률에 더하기
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

    // 수익률 출력하기
    let mut sum: u64 = 0;
    for prize in result {
        sum += Prize::get_sum(prize.0, prize.1);
    }

    println!(
        "총 수익률은 {:.1}%입니다.",
        sum as f64 / money as f64 * 100.0
    );
}

fn input_purchase_amount() -> Result<i32, String> {
    println!("구매금액을 입력해 주세요.");
    let mut input = String::new();
    if let Err(_e) = io::stdin().read_line(&mut input) {
        return Err("[ERROR] 잘못된 입력입니다.".to_string());
    }

    let money: i32 = match input.trim().parse::<i32>() {
        Ok(value) => value,
        Err(_e) => {
            return Err("[ERROR] 구매 금액은 숫자만 입력할 수 있습니다.".to_string());
        }
    };

    if money <= 0 || money > 1_000_000_000 {
        return Err("[ERROR] 구매 금액은 0원부터 10억 사이여야 합니다.".to_string());
    }

    if money % 1000 != 0 {
        return Err("[ERROR] 구매 금액은 1000원 단위여야 합니다.".to_string());
    }

    Ok(money)
}

fn input_winning_lotto() -> Result<Lotto, String> {
    println!("당첨 번호를 입력해 주세요.");
    let mut input = String::new();
    if let Err(_e) = io::stdin().read_line(&mut input) {
        return Err("[ERROR] 잘못된 입력입니다.".to_string());
    }

    let parsed = input.split(",");

    let mut lotto_numbers: Vec<i32> = Vec::new();

    for number in parsed {
        match number.trim().parse() {
            Ok(value) => lotto_numbers.push(value),
            Err(_e) => return Err("[ERROR] 로또 번호는 숫자여야 합니다.".to_string()),
        }
    }

    match Lotto::new(lotto_numbers) {
        Ok(value) => Ok(value),
        Err(_e) => Err(_e),
    }
}

fn input_bonus_lotto() -> Result<BonusNumber, String> {
    println!("보너스 번호를 입력해 주세요.");
    let mut input = String::new();
    if let Err(_e) = io::stdin().read_line(&mut input) {
        return Err("[ERROR] 잘못된 입력입니다.".to_string());
    }

    let bonus_number = match input.trim().parse::<i32>() {
        Ok(value) => value,
        Err(_e) => return Err("[ERROR] 보너스 번호는 숫자만 입력해야 합니다.".to_string()),
    };

    match BonusNumber::new(bonus_number) {
        Ok(value) => Ok(value),
        Err(_e) => Err(_e),
    }
}
