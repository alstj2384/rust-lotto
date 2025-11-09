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
