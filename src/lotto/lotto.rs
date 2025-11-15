use rand::Rng;
use std::fmt::Display;

pub struct Lotto {
    pub lotto_numbers: [i8; 6],
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

        Ok(Lotto {
            lotto_numbers: lotto_numbers,
        })
    }

    pub fn contains(&self, number: &i8) -> bool {
        self.lotto_numbers.contains(number)
    }

    pub fn get_count(&self, lotto: &Lotto) -> i32 {
        let mut count = 0;
        for target in &lotto.lotto_numbers {
            if self.contains(target) {
                count += 1;
            }
        }
        count
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
        // let mut vec: Vec<i8> = Vec::new();
        let mut vec = [0i8; 6];

        for i in 0..6 {
            let rand_number = rng.gen_range(1..=45);
            if vec.contains(&rand_number) {
                continue;
            }
            vec[i] = rand_number;
        }
        // while vec.len() != 6 {
        //     let rand_number = rng.gen_range(1..=45);
        //     if vec.contains(&rand_number) {
        //         continue;
        //     }
        //     vec.push(rand_number);
        // }
        vec.sort();
        Lotto { lotto_numbers: vec }
    }

    fn join_by_delimiter(&self) -> String {
        let joined = self
            .lotto_numbers
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        joined
    }
}

impl Display for Lotto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.join_by_delimiter();
        write!(f, "[{}]", joined)
    }
}
