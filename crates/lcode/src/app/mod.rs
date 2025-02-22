mod dispatch;
mod edit;
mod impl_app;
mod info;
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
    Info,
}

impl From<TuiIndex> for usize {
    fn from(val: TuiIndex) -> Self {
        match val {
            TuiIndex::Select => 0,
            TuiIndex::Edit => 1,
            TuiIndex::Topic => 2,
            TuiIndex::Info => 3,
        }
    }
}

impl TuiIndex {
    fn next(&mut self) {
        *self = match self {
            Self::Select => Self::Edit,
            Self::Edit => Self::Topic,
            Self::Topic => Self::Info,
            Self::Info => Self::Select,
        };
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Select => Self::Info,
            Self::Edit => Self::Select,
            Self::Topic => Self::Edit,
            Self::Info => Self::Topic,
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
    #[expect(dead_code, reason = "todo use it")]
    Visual,

    /// not enter input
    #[default]
    OutEdit,
}
