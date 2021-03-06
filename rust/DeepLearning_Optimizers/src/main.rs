#![cfg_attr(feature = "strict", deny(warnings))]

use std::env;

extern crate deeplearning_optimizers;
use deeplearning_optimizers::optimizers::*;

static VERBOSE: bool = true;

// Z = 0.1*X**3-0.3*X*(Y**2)+200
// Dx= 0.3*x^2 - 0.3y^2
// Dy= -0.6*x*y

fn example_ground_truth(x: Vec<f32>) -> f32 {
    0.1 * x[0].powf(3.0) - 0.3 * x[0] * x[1].powf(2.0) + 200.0
}

fn example_derrivative_x(x: Vec<f32>) -> f32 {
    0.3 * x[0].powf(2.0) - 0.3 * x[1].powf(2.0)
}
fn example_derrivative_y(x: Vec<f32>) -> f32 {
    -0.6 * x[0] * x[1]
}

fn epoch_callback_printer(epoch: i32, current_x: Vec<f32>, loss: f32, momentum: Option<f32>) {
    if VERBOSE {
        match momentum {
            Some(x) => println!(
                "Epoch: {}, current pos: {:?}, loss: {}, momentum: {}",
                epoch, current_x, loss, x
            ),
            None => println!("{:?}, {}", current_x, loss),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let no_summary = if args.len() > 1 && (&args[1] == "--no_summary") {
        true
    } else {
        false
    };
    let algo = if args.len() > 2 { &args[2] } else { "" };

    let example_derrivatives: Vec<fn(Vec<f32>) -> f32> =
        vec![example_derrivative_x, example_derrivative_y];
    let start_: Vec<f32> = vec![-2.5, -5.0];

    let config_metadata = ConfigMetadata {
        start: start_,
        step_size: 0.1,
        precision: 0.00001,
        max_epochs: 10000,
        ground_truth: example_ground_truth,
        derrivatives: example_derrivatives,
        epoch_printer: epoch_callback_printer,
    };

    if algo == "--algo=gd" {
        let gradient_descent = GrandientDescent {};
        let (found, minimum_x, epochs) = gradient_descent.run(&config_metadata);
        if !no_summary {
            println!(
                "GD: Solution: {:?}, minimum pos found at: {:?}, epochs: {}",
                found, minimum_x, epochs
            );
        }
    }

    if algo == "--algo=rmsprop" {
        let rms_prop = RMSProp {};
        let decay = 0.9;
        let (found, minimum_x, epochs) = rms_prop.run(&config_metadata, decay);
        if !no_summary {
            println!(
                "RMSProp: Solution: {:?}, minimum X found at: {:?}, epochs: {}",
                found, minimum_x, epochs
            );
        }
    }

    if algo == "--algo=rmsprop_momentum" {
        let rms_prop_momentum = RMSPropMomentum {};
        let decay = 0.9;
        let mu = 0.9;
        let (found, minimum_x, epochs) = rms_prop_momentum.run(&config_metadata, decay, mu);
        if !no_summary {
            println!(
                "RMSProm with momentum: Solution: {:?}, minimum X found at: {:?}, epochs: {}",
                found, minimum_x, epochs
            );
        }
    }
}
