pub struct ConfigMetadata {
    pub start_x: f32,
    pub step_size: f32,
    pub precision: f32,
    pub max_epochs: i32,
    pub derrivative: fn(f32) -> f32,
    pub epoch_printer: fn(i32, f32, f32, Option<f32>),
}

pub struct GrandientDescent {}

impl GrandientDescent {
    pub fn run(config_metadata: &ConfigMetadata) -> (bool, f32, i32) {
        let mut next = config_metadata.start_x;
        let mut current_x = 0.0;
        let mut found = false;
        let mut epochs = 0;
        for epoch in 0..config_metadata.max_epochs {
            current_x = next;
            next = current_x - config_metadata.step_size * (config_metadata.derrivative)(current_x);
            let loss = next - current_x;

            (config_metadata.epoch_printer)(epoch, current_x, loss, None);
            epochs += 1;
            if loss.abs() <= config_metadata.precision {
                found = true;
                break;
            }
        }
        (found, current_x, epochs)
    }
}

pub struct RMSProp {}

impl RMSProp {
    pub fn run(config_metadata: &ConfigMetadata, decay: f32) -> (bool, f32, i32) {
        let mut next = config_metadata.start_x;
        let mut current_x = 0.0;
        let mut found = false;
        let mut dx_mean_sqr = 0.0;
        let epsilon = 0.00000001; // neccessary for numerical stability, avoid div with 0
        let mut epochs = 0;

        for epoch in 0..config_metadata.max_epochs {
            current_x = next;
            let dx = (config_metadata.derrivative)(current_x);
            dx_mean_sqr = decay * dx_mean_sqr + (1.0 - decay) * dx.powf(2.0);
            next = current_x - config_metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);

            let loss = next - current_x;

            (config_metadata.epoch_printer)(epoch, current_x, loss, None);
            epochs += 1;
            if loss.abs() <= config_metadata.precision {
                found = true;
                break;
            }
        }
        (found, current_x, epochs)
    }
}

pub struct RMSPropMomentum {}

impl RMSPropMomentum {
    pub fn run(config_metadata: ConfigMetadata, decay: f32, mu: f32) -> (bool, f32, i32) {
        let mut next = config_metadata.start_x;
        let mut current_x = 0.0;
        let mut found = false;
        let mut dx_mean_sqr = 0.0;
        let epsilon = 0.00000001; // neccessary for numerical stability, avoid div with 0
        let mut momentum = 0.0;
        let mut epochs = 0;

        for epoch in 0..config_metadata.max_epochs {
            current_x = next;
            let dx = (config_metadata.derrivative)(current_x);
            dx_mean_sqr = decay * dx_mean_sqr + (1.0 - decay) * dx.powf(2.0);
            momentum =
                mu * momentum + config_metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);
            next = current_x - momentum;

            let loss = next - current_x;

            (config_metadata.epoch_printer)(epoch, current_x, loss, Some(momentum));
            epochs += 1;
            if loss.abs() <= config_metadata.precision {
                found = true;
                break;
            }
        }
        (found, current_x, epochs)
    }
}
