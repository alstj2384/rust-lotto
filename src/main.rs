use std::io;

fn main() {
    // 구매할 로또 금액 입력받기
    let mut money = String::new();
    io::stdin()
        .read_line(&mut money)
        .expect("구매 금액이 잘못되었습니다!");

    money = money
        .trim()
        .parse()
        .expect("구매 금액은 숫자만 입력할 수 있습니다!");

    println!("{}", money);
    // 생성된 로또 개수 보여주기
    // 생성된 로또 보여주기

    // 당첨 로또 번호 입력받기
    // 보너스 번호 입력받기

    // 결과를 출력하기
    // 수익률 출력하기
}
