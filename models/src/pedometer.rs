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

#[cfg(test)]
mod tests {
    use super::*;
    use accelerometer::vector::F32x3;

    // TODO: 平方根のテストする方法探す
    // #[test]
    // fn test_get_composite_accel() {
    //     let normalized_accel = F32x3::new(1.0, 1.0, 1.0);

    //     let composite_accel = Pedometer::get_composite_accel(normalized_accel);

    //     assert_eq!(composite_accel, 1.73205080756887729352);
    // }

    #[test]
    fn test_set_threshold() {
        // TODO: f32のasset_eq!やる方法探す

        let mut pedometer = Pedometer::new();
        let composite_accel = 2.0;

        for count in 1..50 {
            Pedometer::set_threshold(&mut pedometer, composite_accel);

            if count == 50 {
                // assert_eq!(pedometer.threshold, 2.0);
                // assert_eq!(pedometer.hysteresis, 0.4);
                // assert_eq!(pedometer.total_composite_accel, 0.0);
                assert_eq!(pedometer.sample_count, 0);
            } else {
                // assert_eq!(pedometer.threshold, 1.5);
                // assert_eq!(pedometer.hysteresis, 0.15);
                // assert_eq!(pedometer.total_composite_accel, 2.0 * count as f32);
                assert_eq!(pedometer.sample_count, count);
            }
        }
    }

    #[test]
    fn test_set_state() {
        let mut pedometer1 = Pedometer::new();
        let large_composite_accel = 2.0;

        Pedometer::set_state(&mut pedometer1, large_composite_accel);

        assert!(pedometer1.state);

        let mut pedometer2 = Pedometer::new();
        let small_composite_accel = 1.0;

        Pedometer::set_state(&mut pedometer2, small_composite_accel);

        assert!(!pedometer2.state);
    }

    #[test]
    fn test_set_step_count() {
        let mut pedometer1 = Pedometer::new();
        pedometer1.last_state = false;
        pedometer1.state = false;

        Pedometer::set_step_count(&mut pedometer1);

        assert_eq!(pedometer1.step_count, 0);
        assert!(!pedometer1.last_state);

        let mut pedometer2 = Pedometer::new();
        pedometer2.last_state = false;
        pedometer2.state = true;

        Pedometer::set_step_count(&mut pedometer2);

        assert_eq!(pedometer2.step_count, 1);
        assert!(pedometer2.last_state);

        let mut pedometer3 = Pedometer::new();
        pedometer3.last_state = true;
        pedometer3.state = true;

        Pedometer::set_step_count(&mut pedometer3);

        assert_eq!(pedometer3.step_count, 0);
        assert!(pedometer3.last_state);

        let mut pedometer4 = Pedometer::new();
        pedometer4.last_state = true;
        pedometer4.state = false;

        Pedometer::set_step_count(&mut pedometer4);

        assert_eq!(pedometer4.step_count, 0);
        assert!(!pedometer4.last_state);
    }
}
