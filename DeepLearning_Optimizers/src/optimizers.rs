#[derive(Debug)]
pub enum OptimizerResult {
    Converged,
    NotConverged,
}

pub struct ConfigMetadata {
    pub start: Vec<f32>,
    pub step_size: f32,
    pub precision: f32,
    pub max_epochs: i32,
    pub derrivatives: Vec<fn(f32) -> f32>,
    pub epoch_printer: fn(i32, Vec<f32>, f32, Option<f32>),
}

pub struct GrandientDescent {}

impl GrandientDescent {
    pub fn run(config_metadata: &ConfigMetadata) -> (OptimizerResult, Vec<f32>, i32) {
        let mut next = config_metadata.start.clone();
        let mut current;
        let mut epochs = 0;
        for epoch in 0..config_metadata.max_epochs {
            current = next.clone();
            epochs += 1;
            for dimmension in 0..config_metadata.derrivatives.len() {
                next[dimmension] = current[dimmension]
                    - config_metadata.step_size
                        * (config_metadata.derrivatives[dimmension])(current[dimmension]);
                let loss = next[dimmension] - current[dimmension];

                (config_metadata.epoch_printer)(epoch, current.clone(), loss, None);

                if loss.abs() <= config_metadata.precision {
                    return (OptimizerResult::Converged, current, epochs);
                }
            }
        }
        (OptimizerResult::NotConverged, Vec::with_capacity(0), epochs)
    }
}

pub struct RMSProp {}

impl RMSProp {
    pub fn run(config_metadata: &ConfigMetadata, decay: f32) -> (OptimizerResult, Vec<f32>, i32) {
        let mut next = config_metadata.start.clone();
        let mut current;
        let mut dx_mean_sqr = 0.0;
        let epsilon = 0.00000001; // neccessary for numerical stability, avoid div with 0
        let mut epochs = 0;

        for epoch in 0..config_metadata.max_epochs {
            current = next.clone();
            epochs += 1;
            for dimmension in 0..config_metadata.derrivatives.len() {
                let dx = (config_metadata.derrivatives[dimmension])(current[dimmension]);
                dx_mean_sqr = decay * dx_mean_sqr + (1.0 - decay) * dx.powf(2.0);
                next[dimmension] = current[dimmension]
                    - config_metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);

                let loss = next[dimmension] - current[dimmension];

                (config_metadata.epoch_printer)(epoch, current.clone(), loss, None);
                if loss.abs() <= config_metadata.precision {
                    return (OptimizerResult::Converged, current, epochs);
                }
            }
        }
        (OptimizerResult::NotConverged, Vec::with_capacity(0), epochs)
    }
}

pub struct RMSPropMomentum {}

impl RMSPropMomentum {
    pub fn run(
        config_metadata: &ConfigMetadata,
        decay: f32,
        mu: f32,
    ) -> (OptimizerResult, Vec<f32>, i32) {
        let mut next = config_metadata.start.clone();
        let mut current;
        let mut dx_mean_sqr = 0.0;
        let epsilon = 0.00000001; // neccessary for numerical stability, avoid div with 0
        let mut momentum = 0.0;
        let mut epochs = 0;

        for epoch in 0..config_metadata.max_epochs {
            current = next.clone();
            epochs += 1;
            for dimmension in 0..config_metadata.derrivatives.len() {
                let dx = (config_metadata.derrivatives[dimmension])(current[dimmension]);
                dx_mean_sqr = decay * dx_mean_sqr + (1.0 - decay) * dx.powf(2.0);
                momentum =
                    mu * momentum + config_metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);
                next[dimmension] = current[dimmension] - momentum;

                let loss = next[dimmension] - current[dimmension];

                (config_metadata.epoch_printer)(epoch, current.clone(), loss, Some(momentum));

                if loss.abs() <= config_metadata.precision {
                    return (OptimizerResult::Converged, current, epochs);
                }
            }
        }
        (OptimizerResult::NotConverged, Vec::with_capacity(0), epochs)
    }
}
