mod gradient_descent;
use gradient_descent::GrandientDescent;
use gradient_descent::Metadata;

fn example_derrivative(x: f32) -> f32 {
    4.0 * x.powf(3.0) - 9.0 * x.powf(2.0)
}

fn main() {
    let metadata = Metadata {
        start_x: 6.0,
        step_size: 0.01,
        precision: 0.00001,
        max_epochs: 10000,
        derrivative: example_derrivative,
    };
    let (found, minimum_x) = GrandientDescent::run(metadata);
    println!("Solution : {}, minimum X found at: {}", found, minimum_x);
}
