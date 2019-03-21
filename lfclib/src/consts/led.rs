pub const FRONT_TOP    : u8 = 1;
pub const FRONT_MIDDLE : u8 = 2;
pub const FRONT_BOTTOM : u8 = 3;
pub const BACK_TOP     : u8 = 4;
pub const BACK_MIDDLE  : u8 = 5;
pub const BACK_BOTTOM  : u8 = 6;

pub const FRONT_ALL    : u8 = 65;
pub const BACK_ALL     : u8 = 66;
pub const ALL          : u8 = 255;

#[derive(Copy, Clone)]
pub enum Led {
    FrontTop = 1,
    FrontMiddle = 2,
    FrontBottom = 3,
    BackTop = 4,
    BackMiddle = 5,
    BackBottom = 6,
    FrontAll = 65,
    BackAll = 66,
    All = 255,
}
