///
/// This example parses, sorts and groups the iris dataset
/// and does some simple manipulations.
///
/// Iterators and itertools functionality are used throughout.
///
///
use super::*;
use ndarray::Array;
use rustc_serialize::Decodable;
use std::fmt::Debug;
use std::str::FromStr;
use csv;



pub trait ReadCSV<T>
    where T: UtahNum + Decodable
{
    fn read_csv(file: &'static str) -> Result<DataFrame<T>>;
}

impl<T> ReadCSV<T> for DataFrame<T>
    where T: UtahNum + Decodable + FromStr + Debug
{
    fn read_csv(file: &'static str) -> Result<DataFrame<T>> {
        let mut rdr = csv::Reader::from_path(file).unwrap();
        let columns : Vec<String> = rdr.headers().unwrap().iter().map(|t| t.to_string()).collect();
        let (mut nrow, ncol) = (0, columns.len());
        let mut v: Vec<T> = Vec::new();
        for record in rdr.records() {
            nrow += 1;
            let e: Vec<T> = record.unwrap().iter().map(|t| parse_str::<T>(t)).collect();
            v.extend(e.into_iter())
        }

        let matrix = Array::from_shape_vec((nrow, ncol), v).unwrap();
        DataFrame::new(matrix).columns(&columns[..])
    }
}
fn parse_str<T: FromStr>(text: &str) -> T {
    match text.parse::<T>(){
        Ok(p) => p,
        Err(_e) => panic!("Error in converting the data")
    }

}
