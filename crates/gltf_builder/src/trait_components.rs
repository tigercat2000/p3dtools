use crate::{Vector2, Vector3, Vector4};

pub trait MinMaxComponents {
    type Item;
    fn min_components(&self) -> Option<Self::Item>;
    fn max_components(&self) -> Option<Self::Item>;
}

impl MinMaxComponents for &[Vector2] {
    type Item = Vector2;

    fn min_components(&self) -> Option<Self::Item> {
        self.iter()
            .copied()
            .reduce(|acc, chunk| [acc[0].min(chunk[0]), acc[1].min(chunk[1])])
    }

    fn max_components(&self) -> Option<Self::Item> {
        self.iter()
            .copied()
            .reduce(|acc, chunk| [acc[0].max(chunk[0]), acc[1].max(chunk[1])])
    }
}

impl MinMaxComponents for &[Vector3] {
    type Item = Vector3;

    fn min_components(&self) -> Option<Self::Item> {
        self.iter().copied().reduce(|acc, chunk| {
            [
                acc[0].min(chunk[0]),
                acc[1].min(chunk[1]),
                acc[2].min(chunk[2]),
            ]
        })
    }

    fn max_components(&self) -> Option<Self::Item> {
        self.iter().copied().reduce(|acc, chunk| {
            [
                acc[0].max(chunk[0]),
                acc[1].max(chunk[1]),
                acc[2].max(chunk[2]),
            ]
        })
    }
}

impl MinMaxComponents for &[Vector4] {
    type Item = Vector4;

    fn min_components(&self) -> Option<Self::Item> {
        self.iter().copied().reduce(|acc, chunk| {
            [
                acc[0].min(chunk[0]),
                acc[1].min(chunk[1]),
                acc[2].min(chunk[2]),
                acc[3].min(chunk[3]),
            ]
        })
    }

    fn max_components(&self) -> Option<Self::Item> {
        self.iter().copied().reduce(|acc, chunk| {
            [
                acc[0].max(chunk[0]),
                acc[1].max(chunk[1]),
                acc[2].max(chunk[2]),
                acc[3].max(chunk[3]),
            ]
        })
    }
}
