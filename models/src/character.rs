use crate::{
    meal::Meal,
    pedometer::Pedometer,
    rice_ball::RiceBall
};

pub struct Character {
    pub intimacy: i32,
}

impl Default for Character {
    fn default() -> Self {
        Character::new()
    }
}

impl Character {
    pub fn new() -> Character {
        Character {
            intimacy: 0,
        }
    }
    pub fn eat(&mut self, meal: &mut Meal, rice_ball: &mut RiceBall) {
        // 食事量を親密度に足す
        self.intimacy += meal.amount;
        // 食べた分おにぎりの数を減らす
        rice_ball.amount -= meal.amount;
        // 食事量をリセット
        meal.amount = 0;
    }
    pub fn walk(pedometer: &Pedometer, rice_ball: &mut RiceBall) {
        const FREQUENCY_OF_STEPS: i32 = 10;

        // 歩数計が10歩カウントするごとにおにぎりを1個見つける
        if pedometer.step_count - FREQUENCY_OF_STEPS >= rice_ball.last_step_count {
            // 最後に見つけた歩数カウントを記録する
            rice_ball.last_step_count = pedometer.step_count;
            // おにぎりを1個追加する
            rice_ball.amount += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat() {
        let mut character = Character { intimacy: 0 };
        let mut meal = Meal { amount: 1 };
        let mut rice_ball = RiceBall {
            amount: 10,
            last_step_count: 100
        };

        Character::eat(&mut character, &mut meal, &mut rice_ball);

        assert_eq!(character.intimacy, 1);
        assert_eq!(meal.amount, 0);
        assert_eq!(rice_ball.amount, 9);
    }

    #[test]
    fn test_walk() {
        let mut pedometer = Pedometer {
            sample_count: 0,
            total_composite_accel: 0.0,
            threshold: 1.5,
            hysteresis: 0.15,
            step_count: 0,
            state: false,
            last_state: false
        };
        let mut rice_ball = RiceBall {
            amount: 0,
            last_step_count: 0
        };

        Character::walk(&pedometer, &mut rice_ball);

        assert_eq!(rice_ball.amount, 0);
        assert_eq!(rice_ball.last_step_count, 0);

        pedometer.step_count = 10;

        Character::walk(&pedometer, &mut rice_ball);

        assert_eq!(rice_ball.amount, 1);
        assert_eq!(rice_ball.last_step_count, 10);
    }
}
