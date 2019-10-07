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
fn epoch_print(
    run_metadata: &RunMetadata, config_metadata: &ConfigMetadata, loss: f32, epoch: i32,
) {
    (config_metadata.epoch_printer)(epoch, run_metadata.current.clone(), loss, None);
}

fn non_converged(run_metadata: RunMetadata) -> (OptimizerResult, Vec<f32>, i32) {
    (
        OptimizerResult::NotConverged,
        Vec::with_capacity(0),
        run_metadata.epochs,
    )
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

                epoch_print(&run_metadata, &config_metadata, loss, epoch);

                if loss.abs() <= config_metadata.precision {
                    return (
                        OptimizerResult::Converged,
                        run_metadata.current,
                        run_metadata.epochs,
                    );
                }
            }
        }
        non_converged(run_metadata)
    }

    fn implementation(
        &self, run_metadata: &mut RunMetadata, config_metadata: &ConfigMetadata, dimmension: usize,
    ) -> f32 {
        run_metadata.next[dimmension] = run_metadata.current[dimmension]
            - config_metadata.step_size
                * (config_metadata.derrivatives[dimmension])(run_metadata.current[dimmension]);
        let loss = run_metadata.next[dimmension] - run_metadata.current[dimmension];
        loss
    }
}

pub struct RMSProp {}

impl RMSProp {
    pub fn run(
        self, config_metadata: &ConfigMetadata, decay: f32,
    ) -> (OptimizerResult, Vec<f32>, i32) {
        let mut run_metadata = RunMetadata {
            next: config_metadata.start.clone(),
            current: Vec::with_capacity(0),
            epochs: 0,
        };
        let mut dx_mean_sqr = 0.0;
        for epoch in 0..config_metadata.max_epochs {
            run_metadata.current = run_metadata.next.clone();
            run_metadata.epochs += 1;
            for dimmension in 0..config_metadata.derrivatives.len() {
                let loss = self.implementation(
                    &mut run_metadata,
                    &config_metadata,
                    dimmension,
                    decay,
                    &mut dx_mean_sqr,
                );

                epoch_print(&run_metadata, &config_metadata, loss, epoch);

                if loss.abs() <= config_metadata.precision {
                    return (
                        OptimizerResult::Converged,
                        run_metadata.current,
                        run_metadata.epochs,
                    );
                }
            }
        }
        non_converged(run_metadata)
    }

    fn implementation(
        &self, run_metadata: &mut RunMetadata, config_metadata: &ConfigMetadata, dimmension: usize,
        decay: f32, dx_mean_sqr: &mut f32,
    ) -> f32 {
        let epsilon = 0.00000001; // neccessary for numerical stability, avoid div with 0
        let dx = (config_metadata.derrivatives[dimmension])(run_metadata.current[dimmension]);
        *dx_mean_sqr = decay * (*dx_mean_sqr) + (1.0 - decay) * dx.powf(2.0);
        run_metadata.next[dimmension] = run_metadata.current[dimmension]
            - config_metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);

        let loss = run_metadata.next[dimmension] - run_metadata.current[dimmension];

        loss
    }
}

pub struct RMSPropMomentum {}

impl RMSPropMomentum {
    pub fn run(
        self, config_metadata: &ConfigMetadata, decay: f32, mu: f32,
    ) -> (OptimizerResult, Vec<f32>, i32) {
        let mut run_metadata = RunMetadata {
            next: config_metadata.start.clone(),
            current: Vec::with_capacity(0),
            epochs: 0,
        };
        let mut dx_mean_sqr = 0.0;
        let mut momentum = 0.0;
        for epoch in 0..config_metadata.max_epochs {
            run_metadata.current = run_metadata.next.clone();
            run_metadata.epochs += 1;
            for dimmension in 0..config_metadata.derrivatives.len() {
                let loss = self.implementation(
                    &mut run_metadata,
                    &config_metadata,
                    dimmension,
                    decay,
                    mu,
                    &mut dx_mean_sqr,
                    &mut momentum,
                );

                epoch_print(&run_metadata, &config_metadata, loss, epoch);

                if loss.abs() <= config_metadata.precision {
                    return (
                        OptimizerResult::Converged,
                        run_metadata.current,
                        run_metadata.epochs,
                    );
                }
            }
        }
        non_converged(run_metadata)
    }

    fn implementation(
        &self, run_metadata: &mut RunMetadata, config_metadata: &ConfigMetadata, dimmension: usize,
        decay: f32, mu: f32, dx_mean_sqr: &mut f32, momentum: &mut f32,
    ) -> f32 {
        let epsilon = 0.00000001; // neccessary for numerical stability, avoid div with 0
        let dx = (config_metadata.derrivatives[dimmension])(run_metadata.current[dimmension]);
        *dx_mean_sqr = decay * (*dx_mean_sqr) + (1.0 - decay) * dx.powf(2.0);
        *momentum =
            mu * (*momentum) + config_metadata.step_size * dx / (dx_mean_sqr.sqrt() + epsilon);
        run_metadata.next[dimmension] = run_metadata.current[dimmension] - (*momentum);

        let loss = run_metadata.next[dimmension] - run_metadata.current[dimmension];

        loss
    }
}
