use web_sys::Performance;

pub struct Timer {
    performance: Performance,
    last_frame: f64,
    delta: f64,
}

impl Timer {
    pub fn new() -> Timer {
        let performance = web_sys::window().unwrap().performance().unwrap();
        let last_frame = performance.now();
        let delta = 0.0;
        Timer{
            performance,
            last_frame,
            delta,
        }
    }
    pub fn tick_delta(&mut self) -> f64 {
        let temp: f64 = self.performance.now();
        self.delta = ( temp - self.last_frame ) / 1_000.0;
        self.last_frame = temp;
        self.delta
    }

    pub fn get_delta(&self) -> f64 {
        self.delta
    }
}