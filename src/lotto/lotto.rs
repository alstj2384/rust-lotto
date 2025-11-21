use rand::Rng;
use std::{
    fmt::Display,
    thread::{self, JoinHandle},
};

pub const LOTTO_PRICE: u64 = 1000;
pub const LOTTO_SIZE: usize = 6;
pub const MIN_LOTTO_RANGE: i8 = 1;
pub const MAX_LOTTO_RANGE: i8 = 45;
pub const INVALID_LOTTO_RANGE: &str = "[ERROR] 로또 번호는 1부터 45 사이의 숫자여야 합니다.";
pub const DUPLICATED_LOTTO_NUMBER: &str = "[ERROR] 로또 번호는 중복될 수 없습니다.";
const INVALID_LOTTO_SIZE: &str = "[ERROR] 로또 번호는 6개여야 합니다.";
const DELIMITER: &str = ", ";

pub struct Lotto {
    pub lotto_numbers: [i8; LOTTO_SIZE],
}

impl Lotto {
    pub fn new(lotto_numbers: Vec<i8>) -> Result<Lotto, String> {
        if lotto_numbers.len() != LOTTO_SIZE {
            return Err(INVALID_LOTTO_SIZE.to_string());
        }

        for number in &lotto_numbers {
            if number < &MIN_LOTTO_RANGE || number > &MAX_LOTTO_RANGE {
                return Err(INVALID_LOTTO_RANGE.to_string());
            }
        }

        for i in 0..LOTTO_SIZE {
            for j in i + 1..LOTTO_SIZE {
                if lotto_numbers.get(i) == lotto_numbers.get(j) {
                    return Err(DUPLICATED_LOTTO_NUMBER.to_string());
                }
            }
        }

        let numbers = lotto_numbers.try_into().unwrap();

        Ok(Lotto {
            lotto_numbers: numbers,
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

    pub fn generate_random_lottos(amount: u64, threads: u32) -> Vec<Lotto> {
        // 싱글 스레드 처리
        if threads <= 1 {
            return (0..amount).map(|_| Lotto::generate_by_random()).collect();
        }

        let mut lottos = Vec::with_capacity(amount as usize);
        let val = amount / threads as u64;
        let remainder = amount % threads as u64;

        let mut handles: Vec<JoinHandle<Vec<Lotto>>> = Vec::new();

        // 생성을 설정된 스레드가 병렬 처리
        for i in 0..threads {
            let count = if i == threads - 1 {
                val + remainder
            } else {
                val
            };
            let handle =
                thread::spawn(move || (0..count).map(|_| Lotto::generate_by_random()).collect());
            handles.push(handle);
        }

        // 결과물을 병합
        for handle in handles {
            lottos.extend(handle.join().unwrap());
        }

        lottos
    }

    pub fn size_in_bytes() -> u64 {
        size_of::<[i8; LOTTO_SIZE]>() as u64
    }

    fn generate_by_random() -> Lotto {
        let mut rng = rand::thread_rng();
        let mut vec = [0i8; LOTTO_SIZE];
        let mut i = 0;
        while i < LOTTO_SIZE {
            let rand_number = rng.gen_range(MIN_LOTTO_RANGE..=MAX_LOTTO_RANGE);
            if vec.contains(&rand_number) {
                continue;
            }
            vec[i] = rand_number;
            i += 1;
        }
        vec.sort();
        Lotto { lotto_numbers: vec }
    }

    fn join_by_delimiter(&self) -> String {
        let joined = self
            .lotto_numbers
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER);

        joined
    }
}

// 로또 번호 출력을 위한 Display 트레이트 구현
impl Display for Lotto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.join_by_delimiter();
        write!(f, "[{}]", joined)
    }
}
