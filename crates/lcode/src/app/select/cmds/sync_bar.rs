#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
pub struct BarState {
    pub show:    bool,
    pub percent: f64,
}

impl BarState {
    pub fn update(&mut self, perc: f64) {
        self.percent = perc;
    }
    pub fn close(&mut self) {
        self.show = false;
    }
}
