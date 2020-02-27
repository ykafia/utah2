pub use prelude::*;
pub mod util;
pub mod prelude;
pub mod dataframe;
// mod bench;
pub mod combinators;
// mod experiments;
pub mod implement;
pub mod mixedtypes;
pub mod readcsv;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
