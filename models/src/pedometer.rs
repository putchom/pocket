#![allow(unused_imports)]
use accelerometer::vector::F32x3;
use micromath::F32Ext;

pub struct Pedometer {
    pub sample_count: i32,
    pub total_composite_accel: f32,
    pub threshold: f32,
    pub hysteresis: f32,
    pub step_count: i32,
    pub state: bool,
    pub last_state: bool,
}

impl Default for Pedometer {
    fn default() -> Self {
        Pedometer::new()
    }
}

impl Pedometer {
    pub fn new() -> Pedometer {
        Pedometer {
            sample_count: 0,
            total_composite_accel: 0.0,
            threshold: 1.5,
            hysteresis: 0.15,
            step_count: 0,
            state: false,
            last_state: false,
        }
    }
    pub fn update(&mut self, normalized_accel: F32x3) {
        let composite_accel = Self::get_composite_accel(normalized_accel);

        Self::set_threshold(self, composite_accel);
        Self::set_state(self, composite_accel);
        Self::set_step_count(self);
    }
    // XYZ軸の合成値
    fn get_composite_accel(normalized_accel: F32x3) -> f32 {
        let F32x3 { x, y, z } = normalized_accel;
        (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt()
    }
    // XYZ軸の合成値を、50サンプルごとに平均したものを閾値として設定する。
    // 閾値近辺の値を誤検出しないようにヒステリシスも設定する。
    fn set_threshold(&mut self, composite_accel: f32) {
        const SAMPLE_COUNT_LIMIT: i32 = 50;
        const HYSTERESIS_RANGE: f32 = 5.0;

        if self.sample_count < SAMPLE_COUNT_LIMIT {
            self.total_composite_accel += composite_accel;
            self.sample_count += 1;
        } else {
            self.threshold = self.total_composite_accel / self.sample_count as f32;
            self.hysteresis = self.threshold / HYSTERESIS_RANGE;
            self.total_composite_accel = 0.0;
            self.sample_count = 0;
        }
    }
    // 閾値を判定して状態をセットする
    fn set_state(&mut self, composite_accel: f32) {
        if composite_accel > (self.threshold + self.hysteresis) {
            self.state = true
        } else if composite_accel < (self.threshold - self.hysteresis) {
            self.state = false
        }
    }
    // 状態をもとに歩数をカウントする
    fn set_step_count(&mut self) {
        if !self.last_state && self.state {
            self.step_count += 1;
            self.last_state = self.state;
        } else if self.last_state && !self.state {
            self.last_state = self.state;
        }
    }
}
