use super::*;
use ndarray::{Array2, Array1, ArrayView1, ArrayView2, ArrayViewMut1};
use std::iter::Chain;


#[derive( Clone, Debug, Copy)]
pub enum UtahAxis {
    Row,
    Column,
}

#[derive( Clone, Debug)]
pub enum ImputeStrategy {
    Mean,
}


pub type Column<T> = Array1<T>;
pub type Row<T> = Array1<T>;
pub type RowMut<'a, T> = Array1<&'a mut T>;

pub type Matrix<T> = Array2<T>;
pub type MatrixMut<'a, T> = Array2<&'a mut T>;

pub type ColumnView<'a, T> = ArrayView1<'a, T>;

pub type MatrixView<'a, T> = ArrayView2<'a, T>;

pub type DFIter<'a, T> = DataFrameIterator<'a, T>;
pub type AppendIter<'a, T> = Append<'a, DFIter<'a, T>, T>;
pub type SelectIter<'a, T> = Select<'a, DFIter<'a, T>, T>;
pub type RemoveIter<'a, T> = Remove<'a, DFIter<'a, T>, T>;
pub type InnerJoinIter<'a, T> = InnerJoin<'a, DFIter<'a, T>, T>;
pub type OuterJoinIter<'a, T> = OuterJoin<'a, DFIter<'a, T>, T>;
pub type ConcatIter<'a, T> = Concat<'a, Chain<DFIter<'a, T>, DFIter<'a, T>>, T>;
pub type SumIter<'a, T> = Sum<'a, DFIter<'a, T>, T>;
pub type MaxIter<'a, T> = Max<'a, DFIter<'a, T>, T>;
pub type MinIter<'a, T> = Min<'a, DFIter<'a, T>, T>;
pub type StdevIter<'a, T> = Stdev<'a, DFIter<'a, T>, T>;
pub type MeanIter<'a, T> = Mean<'a, DFIter<'a, T>, T>;
pub type MapDFIter<'a, T, F> = MapDF<'a, T, DataFrameMutIterator<'a, T>, F>;
pub type ImputeIter<'a, T> = Impute<'a, DataFrameMutIterator<'a, T>, T>;

pub type WindowMut<'a, T> = (String, ArrayViewMut1<'a, T>);
pub type Window<'a, T> = (String, ArrayView1<'a, T>);
