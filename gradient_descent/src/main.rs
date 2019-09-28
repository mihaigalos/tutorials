mod gradient_descent;
use gradient_descent::Metadata;
use gradient_descent::GrandientDescent;

fn main() {
    let metadata = Metadata{next_x: 6.0, step_size: 0.01, precision: 0.00001, max_iterations: 10000};
    GrandientDescent::run(metadata);
}