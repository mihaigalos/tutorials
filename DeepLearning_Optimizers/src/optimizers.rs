pub struct Metadata {
    pub start_x: f32,
    pub step_size: f32,
    pub precision: f32,
    pub max_epochs: i32,
    pub derrivative: fn(f32) -> f32,
    pub epoch_printer: fn(i32, f32, f32, Option<f32>),
}

pub struct GrandientDescent {}

impl GrandientDescent {
    pub fn run(metadata: &Metadata) -> (bool, f32) {
        let mut next = metadata.start_x;
        let mut current_x = 0.0;
        let mut found = false;

        for epoch in 0..metadata.max_epochs {
            current_x = next;
            next = current_x - metadata.step_size * (metadata.derrivative)(current_x);
            let loss = next - current_x;

            (metadata.epoch_printer)(epoch, current_x, loss, None);

            if loss.abs() <= metadata.precision {
                found = true;
                break;
            }
        }
        (found, current_x)
    }
}

pub struct RMSProp {}

impl RMSProp {
    pub fn run(metadata: &Metadata, decay: f32) -> (bool, f32) {
        let mut next = metadata.start_x;
        let mut current_x = 0.0;
        let mut found = false;
        let mut dx_mean_sqr = 0.0;
        let epsilon = 0.0000000001; // neccessary for numerical stability, avoid div with 0
        for epoch in 0..metadata.max_epochs {
            current_x = next;
            let dx = (metadata.derrivative)(current_x);
            dx_mean_sqr = decay * dx_mean_sqr + (1.0 - decay) * dx.powf(2.0);
            next = current_x - metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);

            let loss = next - current_x;

            (metadata.epoch_printer)(epoch, current_x, loss, None);

            if loss.abs() <= metadata.precision {
                found = true;
                break;
            }
        }
        (found, current_x)
    }
}

pub struct RMSPropMomentum {}

impl RMSPropMomentum {
    pub fn run(metadata: Metadata, decay: f32, mu: f32) -> (bool, f32) {
        let mut next = metadata.start_x;
        let mut current_x = 0.0;
        let mut found = false;
        let mut dx_mean_sqr = 0.0;
        let epsilon = 0.0000000001; // neccessary for numerical stability, avoid div with 0
        let mut momentum = 0.0;

        for epoch in 0..metadata.max_epochs {
            current_x = next;
            let dx = (metadata.derrivative)(current_x);
            dx_mean_sqr = decay * dx_mean_sqr + (1.0 - decay) * dx.powf(2.0);
            momentum = mu * momentum + metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);
            next = current_x - momentum;

            let loss = next - current_x;

            (metadata.epoch_printer)(epoch, current_x, loss, Some(momentum));

            if loss.abs() <= metadata.precision {
                found = true;
                break;
            }
        }
        (found, current_x)
    }
}
