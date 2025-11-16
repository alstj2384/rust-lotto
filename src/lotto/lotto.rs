use rand::Rng;
use std::fmt::Display;

pub struct Lotto {
    pub bit_lotto_numbers: i64,
}

impl Lotto {
    pub fn new(lotto_numbers: [i8; 6]) -> Result<Lotto, String> {
        if lotto_numbers.len() != 6 {
            return Err("[ERROR] 로또 번호는 6개여야 합니다.".to_string());
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

        let mut bits: i64 = 0;
        for number in lotto_numbers {
            bits = bits | (1 << number);
        }
        Ok(Lotto {
            bit_lotto_numbers: bits,
        })
    }

    pub fn generate_random_lottos(amount: i64) -> Vec<Lotto> {
        let mut lottos: Vec<Lotto> = Vec::new();
        while lottos.len() != amount.try_into().unwrap() {
            lottos.push(Lotto::generate_by_random());
        }
        lottos
    }

    fn generate_by_random() -> Lotto {
        let mut rng = rand::thread_rng();
        let mut bits: i64 = 0;

        let mut count = 0;
        while count < 6 {
            let rand_number = rng.gen_range(1..=45);
            if bits & (1 << rand_number) != 0 {
                continue;
            }
            bits = bits | 1 << rand_number;
            count += 1;
        }
        Lotto {
            bit_lotto_numbers: bits,
        }
    }

    fn join_by_delimiter(&self) -> String {
        // 1~45까지 shift한 값이 0이 아니면, 문자열에 추가한다.
        let mut numbers = [0i8; 6];

        let mut i = 0;
        let mut expr = 1;
        while i < 6 {
            if self.bit_lotto_numbers & (1 << expr) != 0 {
                numbers[i] = expr;
                i += 1;
            }
            expr += 1;
        }

        let joined = numbers
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        joined
    }
    pub fn contains(&self, number: &i64) -> bool {
        self.bit_lotto_numbers & number != 0
    }

    pub fn get_count(&self, target: &Lotto) -> u32 {
        (self.bit_lotto_numbers & target.bit_lotto_numbers).count_ones()
    }
}

impl Display for Lotto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.join_by_delimiter();
        write!(f, "[{}]", joined)
    }
}
