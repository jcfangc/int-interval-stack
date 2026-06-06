use int_interval::traits::IntCO;

mod change_point;
mod height_run;
mod height_segment;
mod height_stats;
mod int_co_stack;
mod stack_window;

pub use change_point::ChangePoint;
pub use height_run::HeightRun;
pub use height_segment::HeightSegment;
pub use height_stats::HeightStats;
pub use int_co_stack::IntCOStack;
pub use stack_window::StackWindow;

pub type I8COStack = IntCOStack<int_interval::I8CO>;
pub type I16COStack = IntCOStack<int_interval::I16CO>;
pub type I32COStack = IntCOStack<int_interval::I32CO>;
pub type I64COStack = IntCOStack<int_interval::I64CO>;
pub type I128COStack = IntCOStack<int_interval::I128CO>;
pub type IsizeCOStack = IntCOStack<int_interval::IsizeCO>;
pub type U8COStack = IntCOStack<int_interval::U8CO>;
pub type U16COStack = IntCOStack<int_interval::U16CO>;
pub type U32COStack = IntCOStack<int_interval::U32CO>;
pub type U64COStack = IntCOStack<int_interval::U64CO>;
pub type U128COStack = IntCOStack<int_interval::U128CO>;
pub type UsizeCOStack = IntCOStack<int_interval::UsizeCO>;
