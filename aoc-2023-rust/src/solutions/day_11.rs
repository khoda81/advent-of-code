use crate::{Day, Part, PuzzleResult, UnimplementedSolution};
use std::io::BufRead;
use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("expected all lines to have the same length")]
    MismatchedLineLengths,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Dim {
    pub size: usize,
    pub stride: usize,
}

impl Dim {
    pub fn new(size: usize, stride: usize) -> Dim {
        Self { size, stride }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct View {
    dims: Vec<usize>,
}

impl View {
    pub fn new(dims: impl Into<Vec<usize>>) -> Self {
        Self { dims: dims.into() }
    }

    pub fn dim(&self, dim_idx: usize) -> Option<Dim> {
        let size = self.dims.get(dim_idx).copied()?;
        Some(Dim::new(size, 1))
    }

    pub fn size(&self) -> usize {
        self.dims.iter().product()
    }
}

pub struct NDTensor<T> {
    data: Box<[T]>,
    view: View,
}

impl<T> NDTensor<T> {
    pub fn dim(&self, dim_idx: usize) -> Dim {
        self.view.dim(dim_idx).unwrap_or(Dim::new(1, self.size()))
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub fn from_data(data: Box<[T]>) -> Self {
        let view = View::new([data.len()]);
        Self { view, data }
    }

    pub fn with_view(self, view: View) -> Result<Self, Self> {
        if view.size() == self.size() {
            Ok(Self { view, ..self })
        } else {
            Err(self)
        }
    }
}

pub fn main<I: BufRead>(mut input: I, part: Part) -> PuzzleResult {
    let mut grid = Vec::new();
    let mut rows = 0;
    for line in input.lines() {
        grid.extend(line?.bytes());
        rows += 1;
    }

    let cols = grid.len() / rows;

    let mut grid = NDTensor::from_data(grid.into_boxed_slice())
        .with_view(View::new([rows, cols]))
        .map_err(|_original_grid| Error::MismatchedLineLengths)?;

    Err(UnimplementedSolution(Day::Day11, part).into())
}
