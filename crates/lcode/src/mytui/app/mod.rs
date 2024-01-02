pub mod inner;
mod dispatch;
mod edit;
mod infos;
mod select;
mod topic;

pub use topic::Tab2Panel;

#[derive(Clone, Copy)]
pub enum TuiIndex {
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

#[derive(Default, Debug, PartialEq, Eq)]
pub enum TuiMode {
    /// input panel
    Normal,
    /// input panel
    Insert,
    /// input panel
    Visual,

    /// not enter input
    #[default]
    OutEdit,
}
