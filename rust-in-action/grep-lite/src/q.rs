#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(value: f64) -> Self {
        if value >= 1.0 {
            Q7(127)
        }else if value <= -1.0  {
            Q7(-128)
        }else {
            Q7((value * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(value: Q7) -> Self {
        (value.0 as f64) * 2_f64.powf(-7.0)
    }
}