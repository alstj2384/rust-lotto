use rand::Rng;
use std::{io, vec};

fn main() {
    // 구매할 로또 금액 입력받기
    let mut money = String::new();
    io::stdin()
        .read_line(&mut money)
        .expect("구매 금액이 잘못되었습니다!");

    let money: i32 = money
        .trim()
        .parse()
        .expect("구매 금액은 숫자만 입력할 수 있습니다!");

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
        lottos.push(vec); // 이동
    }
    // 생성된 로또 개수 보여주기
    // 생성된 로또 보여주기

    // 당첨 로또 번호 입력받기
    let mut wining_lotto_numbers = String::new();
    io::stdin()
        .read_line(&mut wining_lotto_numbers)
        .expect("입력이 잘못되었습니다.");

    let lotto_numbers = wining_lotto_numbers.split(",");

    let mut lotto_numbers_vec: Vec<i32> = Vec::new();

    for number in lotto_numbers {
        lotto_numbers_vec.push(
            number
                .trim()
                .parse()
                .expect("로또 번호는 숫자만 입력해야 합니다."),
        );
    }

    for number in lotto_numbers_vec {
        println!("{}", number);
    }

    // 보너스 번호 입력받기
    let mut bonus_number = String::new();
    io::stdin()
        .read_line(&mut bonus_number)
        .expect("입력이 잘못되었습니다.");

    let bonus_number: i32 = bonus_number
        .trim()
        .parse()
        .expect("보너스 번호는 숫자만 입력해야 합니다.");

    println!("{}", bonus_number);

    // 결과를 출력하기
    // 수익률 출력하기
}
