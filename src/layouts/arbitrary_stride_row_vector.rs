//! Arbitrary stride row vector

use crate::traits::*;
use crate::types::IndexType;

use super::*;

pub struct ArbitraryStrideRowVector {
    dim: IndexType,
    stride: IndexType,
}

impl ArbitraryStrideRowVector {
    pub fn new(dim: IndexType, stride: IndexType) -> Self {
        Self { dim, stride }
    }
}

impl LayoutType for ArbitraryStrideRowVector {
    type IndexLayout = RowVector;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (1, index)
    }

    #[inline]
    fn convert_2d_1d(&self, _row: IndexType, col: IndexType) -> IndexType {
        col
    }

    #[inline]
    fn convert_2d_raw(&self, _row: IndexType, col: IndexType) -> IndexType {
        col * self.stride
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        index * self.stride
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        (1, self.dim)
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        (1, self.stride)
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.dim
    }

    #[inline]
    fn index_layout(&self) -> Self::IndexLayout {
        Self::IndexLayout::new(self.dim().1)
    }
}

impl StridedLayoutType for ArbitraryStrideRowVector {}
