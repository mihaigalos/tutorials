mod gradient_descent;
use gradient_descent::GrandientDescent;
use gradient_descent::Metadata;

fn example_derrivative(x: f32) -> f32 {
    4.0 * x.powf(3.0) - 9.0 * x.powf(2.0)
}

fn main() {
    let metadata = Metadata {
        next_x: 6.0,
        step_size: 0.01,
        precision: 0.00001,
        max_iterations: 10000,
        derrivative: example_derrivative,
    };
    GrandientDescent::run(metadata);
}
