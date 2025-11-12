#[derive(Eq, Hash, PartialEq)]
pub enum Prize {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    None,
}

impl Prize {
    pub fn get_prize(match_count: i32, is_bonus_correct: bool) -> Prize {
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

    pub fn get_sum(prize: Prize, count: i32) -> u64 {
        match prize {
            Prize::First => 2_000_000_000 * count as u64,
            Prize::Second => 30_000_000 * count as u64,
            Prize::Third => 1_500_000 * count as u64,
            Prize::Fourth => 50_000 * count as u64,
            Prize::Fifth => 5_000 * count as u64,
            Prize::None => 0 as u64,
        }
    }
}
