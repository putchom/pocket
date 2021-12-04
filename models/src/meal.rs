pub struct Meal {
    pub amount: i32,
}

impl Default for Meal {
    fn default() -> Self {
        Meal::new()
    }
}

impl Meal {
    pub fn new() -> Meal {
        Meal {
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
        let mut meal = Meal { amount: 0 };
        let rice_ball_max_amount = 10;

        Meal::increase(&mut meal, rice_ball_max_amount);

        assert_eq!(meal.amount, 1);
    }

    #[test]
    fn test_can_not_increase() {
        let mut meal = Meal { amount: 0 };
        let rice_ball_max_amount = 0;

        Meal::increase(&mut meal, rice_ball_max_amount);

        assert_eq!(meal.amount, 0);
    }

    #[test]
    fn test_can_decrease() {
        let mut meal = Meal { amount: 1 };
        
        Meal::decrease(&mut meal);

        assert_eq!(meal.amount, 0);
    }

    #[test]
    fn test_can_not_decrease() {
        let mut meal = Meal { amount: 0 };

        Meal::decrease(&mut meal);

        assert_eq!(meal.amount, 0);
    }
}