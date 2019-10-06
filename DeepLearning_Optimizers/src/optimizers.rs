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

pub struct RunMetadata {
    pub next: Vec<f32>,
    pub current: Vec<f32>,
    pub epochs: i32,
}

pub struct GrandientDescent {}

impl GrandientDescent {
    pub fn run(self, config_metadata: &ConfigMetadata) -> (OptimizerResult, Vec<f32>, i32) {
        let mut run_metadata = RunMetadata {
            next: config_metadata.start.clone(),
            current: Vec::with_capacity(0),
            epochs: 0,
        };
        for epoch in 0..config_metadata.max_epochs {
            run_metadata.current = run_metadata.next.clone();
            run_metadata.epochs += 1;
            for dimmension in 0..config_metadata.derrivatives.len() {
                let loss = self.implementation(&mut run_metadata, &config_metadata, dimmension);

                self.epoch_print(&run_metadata, &config_metadata, loss, epoch);

                if loss.abs() <= config_metadata.precision {
                    return (
                        OptimizerResult::Converged,
                        run_metadata.current,
                        run_metadata.epochs,
                    );
                }
            }
        }
        (
            OptimizerResult::NotConverged,
            Vec::with_capacity(0),
            run_metadata.epochs,
        )
    }

    fn implementation(
        &self,
        run_metadata: &mut RunMetadata,
        config_metadata: &ConfigMetadata,
        dimmension: usize,
    ) -> f32 {
        run_metadata.next[dimmension] = run_metadata.current[dimmension]
            - config_metadata.step_size
                * (config_metadata.derrivatives[dimmension])(run_metadata.current[dimmension]);
        let loss = run_metadata.next[dimmension] - run_metadata.current[dimmension];
        loss
    }
    fn epoch_print(
        &self,
        run_metadata: &RunMetadata,
        config_metadata: &ConfigMetadata,
        loss: f32,
        epoch: i32,
    ) {
        (config_metadata.epoch_printer)(epoch, run_metadata.current.clone(), loss, None);
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
