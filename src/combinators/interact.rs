//! Utah join combinators.

use super::*;
use std::iter::Iterator;
use std::iter::repeat;
use std::collections::HashMap;
use ndarray::{ArrayView1, Array};
use std::iter::Chain;

#[derive(Clone, Debug)]
pub struct Concat<'a, I, T: 'a>
    where I: Iterator<Item = Window<'a, T>>
{
    pub concat_data: I,
    pub concat_other: Vec<String>,
    pub axis: UtahAxis,
}




impl<'a, I, T> Concat<'a, I, T>
    where I: Iterator<Item = Window<'a, T>>
{
    pub fn new(left_df: I,
               right_df: I,
               left_other: Vec<String>,
               axis: UtahAxis)
               -> Concat<'a, Chain<I, I>, T> {

        let it = left_df.chain(right_df);

        Concat {
            concat_data: it,
            concat_other: left_other,
            axis: axis,
        }
    }
}

impl<'a, I, T> Iterator for Concat<'a, I, T>
    where I: Iterator<Item = Window<'a, T>>
{
    type Item = Window<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.concat_data.next()
    }
}

#[derive(Clone)]
pub struct InnerJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum + 'a
{
    pub left: L,
    pub right: HashMap<String, ArrayView1<'a, T>>,
    pub left_columns: Vec<String>,
    pub right_columns: Vec<String>,
}

impl<'a, L, T> InnerJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum + 'a
{
    pub fn new<RI>(left: L,
                   right: RI,
                   left_columns: Vec<String>,
                   right_columns: Vec<String>)
                   -> Self
        where RI: Iterator<Item = Window<'a, T>>
    {
        InnerJoin {
            left: left,
            right: right.collect(),
            left_columns: left_columns,
            right_columns: right_columns,
        }
    }
}



impl<'a, L, T> Iterator for InnerJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum + 'a
{
    type Item = (String, ArrayView1<'a, T>, ArrayView1<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.left.next() {
                Some((k, lv)) => {
                    let rv = self.right.get(&k);

                    match rv {
                        Some(v) => {
                            return Some((k, lv, *v));
                        }
                        None => continue,
                    }
                }
                None => return None,
            }

        }
    }
}

#[derive(Clone)]
pub struct OuterJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum + 'a
{
    left: L,
    right: HashMap<String, ArrayView1<'a, T>>,
    left_columns: Vec<String>,
    right_columns: Vec<String>,
}


impl<'a, L, T> OuterJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum + 'a
{
    pub fn new<RI>(left: L,
                   right: RI,
                   left_columns: Vec<String>,
                   right_columns: Vec<String>)
                   -> Self
        where RI: Iterator<Item = Window<'a, T>>
    {
        OuterJoin {
            left: left,
            right: right.collect(),
            left_columns: left_columns,
            right_columns: right_columns,
        }
    }
}


impl<'a, L, T> Iterator for OuterJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum + 'a
{
    type Item = (String, ArrayView1<'a, T>, Option<ArrayView1<'a, T>>);

    fn next(&mut self) -> Option<Self::Item> {

        match self.left.next() {
            Some((k, lv)) => {
                let rv = self.right.get(&k);
                match rv {
                    Some(v) => return Some((k, lv, Some(*v))),
                    None => Some((k, lv, None)),
                }

            }
            None => None,
        }

    }
}


impl<'a, L, T> ToDataFrame<'a, (String, ArrayView1<'a, T>, ArrayView1<'a, T>), T>
    for InnerJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum{
    fn as_df(self) -> Result<DataFrame<T>> {

        let s = self.clone();
        let right_columns = self.right_columns.clone();
        let left_columns = self.left_columns.clone();
        let mut c = Vec::new();
        let mut n = Vec::new();
        let res_dim = (s.fold(0, |acc, _| acc + 1), left_columns.len() + right_columns.len());


        for (i, j, k) in self {
            let p = j.iter().chain(k.iter()).map(|x| x.to_owned());
            c.extend(p);

            n.push(i.to_owned());
        }

        let columns: Vec<_> = left_columns.iter()
            .chain(right_columns.iter())
            .map(|x| x.to_owned())
            .collect();

        let d = Array::from_shape_vec(res_dim, c).unwrap().mapv(|x| x.to_owned());
        let df = DataFrame::new(d).columns(&columns[..])?.index(&n[..])?;

        Ok(df)



    }

    fn as_matrix(self) -> Result<Matrix<T>> {
        let s = self.clone();
        let right_columns = self.right_columns.clone();
        let left_columns = self.left_columns.clone();
        let mut c = Vec::new();
        let mut n = Vec::new();
        let res_dim = (s.fold(0, |acc, _| acc + 1), left_columns.len() + right_columns.len());


        for (i, j, k) in self {
            let p = j.iter().chain(k.iter()).map(|x| x.to_owned());
            c.extend(p);

            n.push(i.to_owned());
        }


        Ok(Array::from_shape_vec(res_dim, c).unwrap())
    }

    fn as_array(self) -> Result<Row<T>> {
        let mut c = Vec::new();
        for (_, j, k) in self {
            let p = j.iter().chain(k.iter()).map(|x| x.to_owned());
            c.extend(p);
        }
        Ok(Array::from(c))
    }
}


impl<'a, L,T> ToDataFrame<'a, (String, ArrayView1<'a, T>, Option<ArrayView1<'a, T>>), T>
    for OuterJoin<'a, L, T>
    where L: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum{
    fn as_df(self) -> Result<DataFrame<T>> {

        let s = self.clone();
        let right_columns = self.right_columns.clone();
        let left_columns = self.left_columns.clone();
        let mut c = Vec::new();
        let mut n = Vec::new();
        let res_dim = (s.fold(0, |acc, _| acc + 1), left_columns.len() + right_columns.len());

        let r = repeat(T::empty()).take(right_columns.len());
        for (i, j, k) in self {
            c.extend(j.iter().map(|x| x.to_owned()));
            match k {
                Some(z) => c.extend(z.iter().map(|x| x.to_owned())),
                None => c.extend(r.clone()),
            }


            n.push(i.to_owned());
        }

        let columns: Vec<_> = left_columns.iter()
            .chain(right_columns.iter())
            .map(|x| x.to_owned())
            .collect();

        let d = Array::from_shape_vec(res_dim, c).unwrap().mapv(|x| x.to_owned());

        let df = DataFrame::new(d).columns(&columns[..])?.index(&n[..])?;
        Ok(df)



    }
    fn as_matrix(self) -> Result<Matrix<T>> {
        let s = self.clone();
        let right_columns = self.right_columns.clone();
        let left_columns = self.left_columns.clone();
        let mut c = Vec::new();
        let mut n = Vec::new();
        let res_dim = (s.fold(0, |acc, _| acc + 1), left_columns.len() + right_columns.len());

        let r = repeat(T::empty()).take(right_columns.len());
        for (i, j, k) in self {
            c.extend(j.iter().map(|x| x.to_owned()));
            match k {
                Some(z) => c.extend(z.iter().map(|x| x.to_owned())),
                None => c.extend(r.clone()),
            }


            n.push(i.to_owned());
        }

        Ok(Array::from_shape_vec(res_dim, c).unwrap())


    }

    fn as_array(self) -> Result<Row<T>> {
        let right_columns = self.right_columns.clone();
        let mut c = Vec::new();
        let r = repeat(T::empty()).take(right_columns.len());
        for (_, j, k) in self {
            c.extend(j.iter().map(|x| x.to_owned()));
            match k {
                Some(z) => c.extend(z.iter().map(|x| x.to_owned())),
                None => c.extend(r.clone()),
            }
        }
        Ok(Array::from(c))
    }
}



impl<'a, I, T> ToDataFrame<'a, Window<'a, T>, T> for Concat<'a, I, T>
    where I: Iterator<Item = Window<'a, T>> + Clone,
          T: UtahNum
{
    fn as_df(self) -> Result<DataFrame<T>> {

        let s = self.clone();
        let axis = self.axis.clone();
        let other = self.concat_other.clone();
        let mut c = Vec::new();
        let mut n = Vec::new();
        let res_dim = match axis {
            UtahAxis::Row => (s.fold(0, |acc, _| acc + 1), other.len()),
            UtahAxis::Column => (other.len(), s.fold(0, |acc, _| acc + 1)),

        };

        for (i, j) in self {
            c.extend(j.iter().map(|x| x.to_owned()));
            n.push(i.to_owned());
        }

        let d = Array::from_shape_vec(res_dim, c).unwrap().mapv(|x| x.to_owned());

        match axis {
            UtahAxis::Row => {
                let df = DataFrame::new(d).columns(&other[..])?.index(&n[..])?;
                Ok(df)
            }
            UtahAxis::Column => {
                let df = DataFrame::new(d).columns(&n[..])?.index(&other[..])?;
                Ok(df)
            }
        }


    }
    fn as_matrix(self) -> Result<Matrix<T>> {
        let s = self.clone();
        let other = self.concat_other.clone();
        let mut c = Vec::new();
        let mut n = Vec::new();
        let res_dim = match self.axis {
            UtahAxis::Row => (other.len(), s.fold(0, |acc, _| acc + 1)),
            UtahAxis::Column => (s.fold(0, |acc, _| acc + 1), other.len()),

        };

        for (i, j) in self {
            c.extend(j.iter().map(|x| x.to_owned()));
            n.push(i.to_owned());
        }

        Ok(Array::from_shape_vec(res_dim, c).unwrap())


    }

    fn as_array(self) -> Result<Row<T>> {
        let mut c = Vec::new();

        for (_, j) in self {
            c.extend(j.iter().map(|x| x.to_owned()));
        }
        Ok(Array::from(c))
    }
}
