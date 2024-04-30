#![allow(clippy::module_inception)]
use self::theme::{Edit, Info, Select, Tab, TopicTags};

pub(crate) mod theme;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
pub struct Theme {
    pub tab: Tab,

    pub select: Select,
    pub edit:   Edit,
    pub topic:  TopicTags,
    pub info:   Info,
}

// impl Theme {
//     pub fn new(edit: Edit) -> Self {
//         Self { edit }
//     }
// }
