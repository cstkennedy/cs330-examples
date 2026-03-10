pub mod error;
pub mod flooring;
pub mod house;
pub mod io;
pub mod room;

pub mod prelude {
    pub use crate::flooring::Flooring;
    pub use crate::house::House;
    pub use crate::io::HouseParser;
    pub use crate::room::Room;
}
