use rand::Rng;
use std::io;

fn main() {
    // 구매할 로또 금액 입력받기
    let mut money = String::new();
    io::stdin()
        .read_line(&mut money)
        .expect("[ERROR] 구매 금액이 잘못되었습니다!");

    let money: i32 = money
        .trim()
        .parse()
        .expect("[ERROR] 구매 금액은 숫자만 입력할 수 있습니다!");

    // 금액 범위 확인하기
    if money <= 0 || money > 1_000_000_000 {
        panic!("[ERROR] 구매 금액은 0원부터 10억 사이여야 합니다.");
    }
    // 구매 금액 단위 확인하기
    if money % 1000 != 0 {
        panic!("[ERROR] 구매 금액은 1,000원 단위여야 합니다.");
    }

    // 로또 생성하기
    let lotto_amount = money / 1000;

    let mut rng = rand::thread_rng();
    let mut lottos: Vec<Vec<i32>> = Vec::new();
    while lottos.len() != lotto_amount.try_into().unwrap() {
        let mut vec: Vec<i32> = Vec::new();
        // 단일 로또 한 장 만들기
        while vec.len() != 6 {
            let rand_number = rng.gen_range(1..=45);
            if vec.contains(&rand_number) {
                continue;
            }
            vec.push(rand_number);
        }
        vec.sort();
        lottos.push(vec); // 이동
    }

    // 생성된 로또 개수 보여주기
    println!("{}개를 구매했습니다.", lottos.len());

    // 생성된 로또 보여주기
    for lotto in &lottos {
        let joined = lotto
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        print!("[");
        print!("{}", joined);
        println!("]");
    }
    println!();

    // 당첨 로또 번호 입력받기
    println!("당첨 번호를 입력해 주세요.");
    let mut wining_lotto_numbers = String::new();
    io::stdin()
        .read_line(&mut wining_lotto_numbers)
        .expect("[ERROR] 입력이 잘못되었습니다.");

    let lotto_numbers = wining_lotto_numbers.split(",");

    let mut lotto_numbers_vec: Vec<i32> = Vec::new();

    for number in lotto_numbers {
        lotto_numbers_vec.push(
            number
                .trim()
                .parse()
                .expect("[ERROR] 로또 번호는 숫자만 입력해야 합니다."),
        );
    }

    if lotto_numbers_vec.len() != 6 {
        panic!("로또 번호는 6개여야 합니다.");
    }

    for number in &lotto_numbers_vec {
        if number < &1 || number > &45 {
            panic!("[ERROR] 로또 번호는 1부터 45 사이의 숫자여야 합니다.");
        }
    }

    for i in 0..6 {
        for j in i + 1..6 {
            if lotto_numbers_vec.get(i) == lotto_numbers_vec.get(j) {
                panic!("[ERROR] 로또 번호는 중복될 수 없습니다.");
            }
        }
    }

    // 보너스 번호 입력받기
    println!("보너스 번호를 입력해 주세요.");
    let mut bonus_number = String::new();
    io::stdin()
        .read_line(&mut bonus_number)
        .expect("[ERROR] 입력이 잘못되었습니다.");

    let bonus_number: i32 = bonus_number
        .trim()
        .parse()
        .expect("[ERROR] 보너스 번호는 숫자만 입력해야 합니다.");

    if bonus_number < 1 || bonus_number > 45 {
        panic!("[ERROR] 로또 번호는 1부터 45 사이의 숫자여야 합니다.");
    }

    if lotto_numbers_vec.contains(&bonus_number) {
        panic!("[ERROR] 보너스 번호는 로또 번호와 중복될 수 없습니다.");
    }

    println!();

    // 결과를 출력하기
    let mut sum: u64 = 0;
    let mut result: Vec<i32> = Vec::new();
    for _ in 0..6 {
        result.push(0);
    }

    // 1. 로또 번호들과 기본 번호 + 당첨 번호를 비교하기
    for lotto in lottos {
        let mut count = 0;
        let mut is_bonus_correct = false;
        for number in &lotto_numbers_vec {
            if lotto.contains(&number) {
                count += 1;
            }
        }
        if lotto.contains(&bonus_number) {
            is_bonus_correct = true;
        }

        if count == 6 {
            sum += 2_000_000_000;
            if let Some(num) = result.get_mut(0) {
                *num += 1;
            }
        } else if count == 5 && is_bonus_correct {
            sum += 30_000_000;
            if let Some(num) = result.get_mut(1) {
                *num += 1;
            }
        } else if count == 5 {
            sum += 1_500_000;
            if let Some(num) = result.get_mut(2) {
                *num += 1;
            }
        } else if count == 4 {
            sum += 50_000;
            if let Some(num) = result.get_mut(3) {
                *num += 1;
            }
        } else if count == 3 {
            sum += 5_000;
            if let Some(num) = result.get_mut(4) {
                *num += 1;
            }
        } else {
            if let Some(num) = result.get_mut(5) {
                *num += 1;
            }
        }
    }

    println!("당첨 통계");
    println!("---");
    // 2. 당첨 등수를 산정하고 결과 Map에 저장하기, 총 수익률에 더하기
    println!(
        "{}개 일치 ({}원) - {}개",
        3,
        "5,000",
        result.get(4).unwrap()
    );
    println!(
        "{}개 일치 ({}원) - {}개",
        4,
        "50,000",
        result.get(3).unwrap()
    );
    println!(
        "{}개 일치 ({}원) - {}개",
        5,
        "1,500,000",
        result.get(2).unwrap()
    );
    println!(
        "{}개 일치, 보너스 볼 일치 ({}원) - {}개",
        5,
        "30,000,000",
        result.get(1).unwrap()
    );
    println!(
        "{}개 일치 ({}원) - {}개",
        6,
        "2,000,000,000",
        result.get(0).unwrap()
    );

    // 수익률 출력하기
    let profit: f64 = sum as f64 / money as f64 * 100.0;
    println!("총 수익률은 {:.1}%입니다.", profit)
}
