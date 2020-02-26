pub use prelude::*;
mod util;
mod prelude;
mod dataframe;
// mod bench;
mod combinators;
// mod experiments;
mod implement;
mod mixedtypes;
mod readcsv;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
