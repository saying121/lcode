#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
pub struct BarState {
    pub show: bool,
    pub percent: f64,
}

impl BarState {
    pub fn close(&mut self) {
        self.show = false;
    }

    pub fn update(&mut self, percent: f64) {
        self.percent = percent;
    }
}
