// https://easings.net/ja

pub struct Easing;
impl Easing {
    pub fn ease_in_cubic(x: f64) -> f64 {
        x * x * x
    }

    pub fn ease_in_out_cubic(x: f64) -> f64 {
        if x < 0.5 {
            4.0 * x * x * x
        } else {
            1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
        }
    }
}
