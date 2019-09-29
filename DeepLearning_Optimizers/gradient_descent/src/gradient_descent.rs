pub struct Metadata {
    pub start_x: f32,
    pub step_size: f32,
    pub precision: f32,
    pub max_epochs: i32,
    pub derrivative: fn(f32) -> f32,
}

pub struct GrandientDescent {}

impl GrandientDescent {
    pub fn run(metadata: Metadata) -> (bool, f32) {
        let mut next = metadata.start_x;
        let mut current_x = 0.0;
        let mut found = false;

        for epoch in 0..metadata.max_epochs {
            current_x = next;
            next = current_x - metadata.step_size * (metadata.derrivative)(current_x);
            let loss = next - current_x;

            println!("Epoch: {}, current x: {}, loss: {}", epoch, current_x, loss);

            if loss.abs() <= metadata.precision {
                found = true;
                break;
            }
        }
        (found, current_x)
    }
}
