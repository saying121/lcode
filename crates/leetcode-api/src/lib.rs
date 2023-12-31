use std::collections::HashMap;

// pub mod leetcode;
// pub mod urls;

pub type Json = HashMap<&'static str, String>;
#[derive(Default)]
pub enum Suffix {
    Cn,
    #[default]
    En,
}
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
