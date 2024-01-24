mod dispatch;
mod edit;
mod impl_app;
mod infos;
pub mod inner;
mod select;
mod topic;

pub use topic::Tab2Panel;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub enum TuiIndex {
    #[default]
    Select,
    Edit,
    Topic,
    Infos,
}

impl From<TuiIndex> for usize {
    fn from(val: TuiIndex) -> Self {
        match val {
            TuiIndex::Select => 0,
            TuiIndex::Edit => 1,
            TuiIndex::Topic => 2,
            TuiIndex::Infos => 3,
        }
    }
}

impl TuiIndex {
    fn next(&mut self) {
        *self = match self {
            Self::Select => Self::Edit,
            Self::Edit => Self::Topic,
            Self::Topic => Self::Infos,
            Self::Infos => Self::Select,
        };
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Select => Self::Infos,
            Self::Edit => Self::Select,
            Self::Topic => Self::Edit,
            Self::Infos => Self::Topic,
        };
    }
}

#[derive(Clone, Copy)]
#[derive(Default, Debug, PartialEq, Eq)]
pub enum TuiMode {
    /// input panel
    Normal,
    /// input panel
    Insert,
    /// input panel
    #[allow(dead_code)]
    Visual,

    /// not enter input
    #[default]
    OutEdit,
}
