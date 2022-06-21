use rand::prelude::*;

pub fn uuid() -> String {
    let mut rng = rand::thread_rng();
    "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx"
        .chars()
        .map(|c| match c {
            'x' => format!("{:x}", (rng.gen::<f64>() * 16.0).floor() as usize),
            'y' => format!("{:x}", (rng.gen::<f64>() * 4.0).floor() as usize + 8),
            _ => c.to_string(),
        })
        .collect()
}
