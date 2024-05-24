#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
pub struct BarState {
    pub show:     bool,
    pub cur_perc: f64,
}

impl BarState {
    pub fn close(&mut self) {
        self.show = false;
    }

    pub fn update(&mut self, cur_perc: f64) {
        self.cur_perc = cur_perc;
    }
}
