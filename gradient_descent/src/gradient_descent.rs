// Gradient descent, implmented in Rust
pub struct Metadata {
    pub next_x: f32,
    pub step_size: f32,
    pub precision: f32,
    pub max_iterations: i32,
}

pub struct GrandientDescent {}

impl GrandientDescent{
    pub fn run(metadata : Metadata) {
        let mut next = metadata.next_x;
        for i in 0 .. metadata.max_iterations {
            let current_x = next;
            next = current_x - metadata.step_size * derrivative(&current_x);

            let step = next - current_x;
            println!("iteration: {}, current x: {}, step: {}", i, current_x, step);
            if step.abs() <= metadata.precision {
                break;
            }
        }    
    }
}

fn derrivative(x: &f32) -> f32 {
    4.0 * x.powf(3.0) - 9.0 * x.powf(2.0)
}

