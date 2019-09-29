mod optimizers;
use optimizers::GrandientDescent;
use optimizers::ConfigMetadata;
use optimizers::RMSProp;
use optimizers::RMSPropMomentum;

static VERBOSE: bool = false;

fn example_derrivative(x: f32) -> f32 {
    4.0 * x.powf(3.0) - 9.0 * x.powf(2.0)
}

fn epoch_callback_printer(epoch: i32, current_x: f32, loss: f32, momentum: Option<f32>) {
    if VERBOSE {
        match momentum {
            Some(x) => println!(
                "Epoch: {}, current x: {}, loss: {}, momentum: {}",
                epoch, current_x, loss, x
            ),
            None => println!("Epoch: {}, current x: {}, loss: {}", epoch, current_x, loss),
        }
    }
}

fn main() {
    let config_metadata = ConfigMetadata {
        start_x: 6.0,
        step_size: 0.01,
        precision: 0.00001,
        max_epochs: 10000,
        derrivative: example_derrivative,
        epoch_printer: epoch_callback_printer,
    };

    let (found, minimum_x, epochs) = GrandientDescent::run(&config_metadata);
    println!(
        "GD: Solution: {}, minimum X found at: {}, epochs: {}",
        found, minimum_x, epochs
    );

    let decay = 0.9;
    let (found, minimum_x, epochs) = RMSProp::run(&config_metadata, decay);
    println!(
        "RMSProp: Solution: {}, minimum X found at: {}, epochs: {}",
        found, minimum_x, epochs
    );

    let decay = 0.9;
    let mu = 0.9;
    let (found, minimum_x, epochs) = RMSPropMomentum::run(config_metadata, decay, mu);
    println!(
        "RMSProm with momentum: Solution: {}, minimum X found at: {}, epochs: {}",
        found, minimum_x, epochs
    );
}
