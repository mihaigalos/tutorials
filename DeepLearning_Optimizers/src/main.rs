mod optimizers;
use optimizers::ConfigMetadata;
use optimizers::GrandientDescent;
// use optimizers::RMSProp;
// use optimizers::RMSPropMomentum;

static VERBOSE: bool = true;

// Z = 0.1*X**3-0.3*X*(Y**2)+0.9*X*Y
// Dx= 0.3*x^2 - 0.3y^2 + 0.9y
// Dy= -0.6y + 0.9x
fn example_derrivative_x(x: Vec<f32>) -> f32 {
    0.3 * x[0].powf(2.0) - 0.3 * x[1].powf(2.0) + 0.9*x[1]
}
fn example_derrivative_y(x: Vec<f32>) -> f32 {
    -0.6*x[1] + 0.9*x[0]
}

fn epoch_callback_printer(epoch: i32, current_x: Vec<f32>, loss: f32, momentum: Option<f32>) {
    if VERBOSE {
        match momentum {
            Some(x) => println!(
                "Epoch: {}, current pos: {:?}, loss: {}, momentum: {}",
                epoch, current_x, loss, x
            ),
            None => println!(
                // "Epoch: {}, current pos: {:?}, loss: {}",
                "{:?}",
                current_x
            ),
        }
    }
}

fn main() {
    let example_derrivatives: Vec<fn(Vec<f32>) -> f32> = vec![example_derrivative_x, example_derrivative_y];
    let start_: Vec<f32> = vec![0.0,8.0];

    let config_metadata = ConfigMetadata {
        start: start_,
        step_size: 0.01,
        precision: 0.00001,
        max_epochs: 10000,
        derrivatives: example_derrivatives,
        epoch_printer: epoch_callback_printer,
    };

    let gradient_descent = GrandientDescent {};
    let (found, minimum_x, epochs) = gradient_descent.run(&config_metadata);
    println!(
        "GD: Solution: {:?}, minimum pos found at: {:?}, epochs: {}",
        found, minimum_x, epochs
    );

    // let rms_prop = RMSProp {};
    // let decay = 0.9;
    // let (found, minimum_x, epochs) = rms_prop.run(&config_metadata, decay);
    // println!(
    //     "RMSProp: Solution: {:?}, minimum X found at: {:?}, epochs: {}",
    //     found, minimum_x, epochs
    // );

    // let rms_prop_momentum = RMSPropMomentum {};
    // let decay = 0.9;
    // let mu = 0.9;
    // let (found, minimum_x, epochs) = rms_prop_momentum.run(&config_metadata, decay, mu);
    // println!(
    //     "RMSProm with momentum: Solution: {:?}, minimum X found at: {:?}, epochs: {}",
    //     found, minimum_x, epochs
    // );
}
