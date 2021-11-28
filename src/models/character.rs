use crate::models::{
    meal::Meal,
    pedometer::Pedometer,
    rice_ball::RiceBall
};

pub struct Character {
    pub intimacy: i32,
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
