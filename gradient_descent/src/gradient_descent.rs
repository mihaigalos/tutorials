// Gradient descent, implmented in Rust

fn derrivative(x: &f32) -> f32 {
    4.0 * x.powf(3.0) - 9.0 * x.powf(2.0)
}

fn main() {
    let mut next_x: f32 = 6.0; // We start the search at x=6
    let step_size: f32 = 0.01;
    let precision: f32 = 0.00001;
    let max_interations: i32 = 10000;

    for i in 0..max_interations {
        let current_x = next_x;
        next_x = current_x - step_size * derrivative(&current_x);

        let step = next_x - current_x;
        println!("iteration: {}, current x: {}, step: {}", i, current_x, step);
        if step.abs() <= precision {
            break;
        }
    }
}
