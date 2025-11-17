use crate::{
    Lotto,
    lotto::lotto::{INVALID_LOTTO_RANGE, MAX_LOTTO_RANGE, MIN_LOTTO_RANGE},
};

const DUPLICATED_BONUS_NUMBER: &str = "[ERROR] 보너스 번호는 로또 번호와 중복될 수 없습니다.";
pub struct WinningLotto {
    winning_numbers: Lotto,
    bonus_number: BonusNumber,
}

pub struct BonusNumber {
    bonus_number: i8,
}

impl BonusNumber {
    pub fn new(bonus_number: i8) -> Result<BonusNumber, String> {
        if bonus_number < MIN_LOTTO_RANGE || bonus_number > MAX_LOTTO_RANGE {
            return Err(INVALID_LOTTO_RANGE.to_string());
        }
        Ok(BonusNumber {
            bonus_number: bonus_number,
        })
    }

    fn bonus_number(&self) -> &i8 {
        &self.bonus_number
    }
}

impl WinningLotto {
    pub fn new(lotto: Lotto, bonus_number: BonusNumber) -> Result<WinningLotto, String> {
        if lotto.contains(bonus_number.bonus_number()) {
            return Err(DUPLICATED_BONUS_NUMBER.to_string());
        }
        Ok(WinningLotto {
            winning_numbers: lotto,
            bonus_number: bonus_number,
        })
    }

    pub fn get_result(&self, lotto: Lotto) -> (i32, bool) {
        let count = self.winning_numbers.get_count(&lotto);
        let is_bonus_correct = self
            .winning_numbers
            .contains(self.bonus_number.bonus_number());

        (count, is_bonus_correct)
    }
}
