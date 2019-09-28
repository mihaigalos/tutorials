mod rmsprop;
use rmsprop::Metadata;
use rmsprop::RMSProp;

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
    let decay = 0.9;
    let (found, minimum_x) = RMSProp::run(metadata, decay);
    println!("Solution : {}, minimum X found at: {}", found, minimum_x);
}
