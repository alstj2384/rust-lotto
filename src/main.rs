use rand::Rng;
use std::collections::HashMap;
use std::fmt::Display;
use std::io;
struct Lotto {
    lotto_numbers: Vec<i32>,
}

impl Display for Lotto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.join_by_delimiter();
        write!(f, "[{}]", joined)
    }
}

impl Lotto {
    fn join_by_delimiter(&self) -> String {
        let joined = self
            .lotto_numbers
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        joined
    }

    fn generate_by_random() -> Lotto {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = Vec::new();
        while vec.len() != 6 {
            let rand_number = rng.gen_range(1..=45);
            if vec.contains(&rand_number) {
                continue;
            }
            vec.push(rand_number);
        }
        vec.sort();
        Lotto { lotto_numbers: vec }
    }
    fn contains(&self, number: &i32) -> bool {
        self.lotto_numbers.contains(number)
    }

    fn get_count(&self, lotto: &Lotto) -> i32 {
        let mut count = 0;
        for target in &lotto.lotto_numbers {
            if self.contains(target) {
                count += 1;
            }
        }
        count
    }

    fn new(lotto_numbers: Vec<i32>) -> Result<Lotto, String> {
        if lotto_numbers.len() != 6 {
            return Err("로또 번호는 6개여야 합니다.".to_string());
        }

        for number in &lotto_numbers {
            if number < &1 || number > &45 {
                return Err("[ERROR] 로또 번호는 1부터 45 사이의 숫자여야 합니다.".to_string());
            }
        }

        for i in 0..lotto_numbers.len() {
            for j in i + 1..lotto_numbers.len() {
                if lotto_numbers.get(i) == lotto_numbers.get(j) {
                    return Err("[ERROR] 로또 번호는 중복될 수 없습니다.".to_string());
                }
            }
        }

        Ok(Lotto {
            lotto_numbers: lotto_numbers,
        })
    }
}

struct WinningLotto {
    winning_numbers: Lotto,
    bonus_number: BonusNumber,
}

struct BonusNumber {
    bonus_number: i32,
}

impl BonusNumber {
    fn new(bonus_number: i32) -> Result<BonusNumber, String> {
        if bonus_number < 1 || bonus_number > 45 {
            return Err("[ERROR] 로또 번호는 1부터 45 사이의 숫자여야 합니다.".to_string());
        }
        Ok(BonusNumber {
            bonus_number: bonus_number,
        })
    }

    fn bonus_number(&self) -> &i32 {
        &self.bonus_number
    }
}

impl WinningLotto {
    fn new(lotto: Lotto, bonus_number: BonusNumber) -> Result<WinningLotto, String> {
        if lotto.contains(bonus_number.bonus_number()) {
            return Err("[ERROR] 보너스 번호는 로또 번호와 중복될 수 없습니다.".to_string());
        }
        Ok(WinningLotto {
            winning_numbers: lotto,
            bonus_number: bonus_number,
        })
    }

    fn get_result(&self, lotto: Lotto) -> (i32, bool) {
        let count = self.winning_numbers.get_count(&lotto);
        let is_bonus_correct = self
            .winning_numbers
            .contains(self.bonus_number.bonus_number());

        (count, is_bonus_correct)
    }
}
#[derive(Eq, Hash, PartialEq)]
enum Prize {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    None,
}

impl Prize {
    fn get_prize(match_count: i32, is_bonus_correct: bool) -> Prize {
        if match_count == 6 {
            return Prize::First;
        } else if match_count == 5 && is_bonus_correct {
            return Prize::Second;
        } else if match_count == 5 {
            return Prize::Third;
        } else if match_count == 4 {
            return Prize::Fourth;
        } else if match_count == 3 {
            return Prize::Fifth;
        } else {
            return Prize::None;
        }
    }
}

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
    // let profit: f64 = sum as f64 / money as f64 * 100.0;
    // println!("총 수익률은 {:.1}%입니다.", profit)
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
