pub struct Bet {
    pub amount: i32,
}

impl Default for Bet {
    fn default() -> Self {
        Bet::new()
    }
}

impl Bet {
    pub fn new() -> Bet {
        Bet {
            amount: 0
        }
    }
    pub fn increase(&mut self, max: i32) {
        if self.amount < max {
            self.amount += 1;
        }
    }
    pub fn decrease(&mut self) {
        if self.amount > 0 {
            self.amount -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_increase() {
        let mut meal = Bet { amount: 0 };
        let rice_ball_max_amount = 10;

        Bet::increase(&mut meal, rice_ball_max_amount);

        assert_eq!(meal.amount, 1);
    }

    #[test]
    fn test_can_not_increase() {
        let mut meal = Bet { amount: 0 };
        let rice_ball_max_amount = 0;

        Bet::increase(&mut meal, rice_ball_max_amount);

        assert_eq!(meal.amount, 0);
    }

    #[test]
    fn test_can_decrease() {
        let mut meal = Bet { amount: 1 };
        
        Bet::decrease(&mut meal);

        assert_eq!(meal.amount, 0);
    }

    #[test]
    fn test_can_not_decrease() {
        let mut meal = Bet { amount: 0 };

        Bet::decrease(&mut meal);

        assert_eq!(meal.amount, 0);
    }
}