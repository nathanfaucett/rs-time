use chrono::UTC;


pub struct Time {
    start_time: f64,
    last_time: f64,
    current_time: f64,

    fixed: f64,
    fixed_delta: f64,
    scale: f64,

    fps_frame: usize,
    fps_last_time: f64,
    fps: f64,

    delta_time: f64,
    min_delta_time: f64,
    max_delta_time: f64,

    frame: usize,
}

impl Time {
    pub fn new() -> Self {
        let start_time = Self::stamp();

        Time {
            start_time: start_time,
            last_time: start_time,
            current_time: start_time,

            fixed: 1f64 / 60f64,
            fixed_delta: 1f64 / 60f64,
            scale: 1f64,

            fps_frame: 0usize,
            fps_last_time: 0f64,
            fps: 60f64,

            delta_time: 1f64 / 60f64,
            min_delta_time: 0.000001f64,
            max_delta_time: 1f64,

            frame: 0usize,
        }
    }

    pub fn update(&mut self) -> &mut Self {

        self.frame = self.frame + 1usize;

        self.last_time = self.current_time;
        self.current_time = Self::stamp() - self.start_time;

        self.fps_frame = self.fps_frame + 1usize;
        if self.fps_last_time + 1f64 < self.current_time {
            self.fps = self.fps_frame as f64 / (self.current_time - self.fps_last_time);
            self.fps_last_time = self.current_time;
            self.fps_frame = 0usize;
        }

        let delta = (self.current_time - self.last_time) * self.scale;
        self.delta_time = {
            if delta < self.min_delta_time { self.min_delta_time }
            else if delta > self.max_delta_time { self.max_delta_time }
            else { delta }
        };

        self
    }

    pub fn set_scale(&mut self, scale: f64) -> &mut Self {
        self.scale = scale;
        self.fixed_delta = self.fixed * scale;
        self
    }
    pub fn scale(&mut self) -> f64 { self.scale }

    pub fn set_fixed_delta(&mut self, fixed_delta: f64) -> &mut Self {
        self.fixed = fixed_delta;
        self.fixed_delta = self.fixed * self.scale;
        self
    }
    pub fn fixed_delta(&mut self) -> f64 { self.fixed_delta }

    pub fn set_min_delta_time(&mut self, min_delta_time: f64) -> &mut Self {
        self.min_delta_time = min_delta_time;
        self
    }
    pub fn set_max_delta_time(&mut self, max_delta_time: f64) -> &mut Self {
        self.max_delta_time = max_delta_time;
        self
    }

    pub fn fps(&self) -> f64 { self.fps }
    pub fn frame(&self) -> usize { self.frame }

    pub fn start_time(&self) -> f64 { self.start_time }
    pub fn current_time(&self) -> f64 { self.current_time }
    pub fn delta_time(&self) -> f64 { self.delta_time }

    pub fn now(&self) -> f64 { Self::stamp() - self.start_time }

    pub fn stamp() -> f64 {
        let current_time = UTC::now();
        (current_time.timestamp() as f64) +
        (current_time.timestamp_subsec_nanos() as f64 / 1000000000f64)
    }
}
