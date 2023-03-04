use crate::{types::Matrix4, Vector2, Vector3, Vector4};

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

impl MinMaxComponents for &[[u32; 4]] {
    type Item = [u32; 4];

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

impl MinMaxComponents for &[[u16; 4]] {
    type Item = [u16; 4];

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

impl MinMaxComponents for &[Matrix4] {
    type Item = Matrix4;

    fn min_components(&self) -> Option<Self::Item> {
        self.iter().copied().reduce(|acc, chunk| {
            [
                acc[0].min(chunk[0]),
                acc[1].min(chunk[1]),
                acc[2].min(chunk[2]),
                acc[3].min(chunk[3]),
                acc[4].min(chunk[4]),
                acc[5].min(chunk[5]),
                acc[6].min(chunk[6]),
                acc[7].min(chunk[7]),
                acc[8].min(chunk[8]),
                acc[9].min(chunk[9]),
                acc[10].min(chunk[10]),
                acc[11].min(chunk[11]),
                acc[12].min(chunk[12]),
                acc[13].min(chunk[13]),
                acc[14].min(chunk[14]),
                acc[15].min(chunk[15]),
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
                acc[4].max(chunk[4]),
                acc[5].max(chunk[5]),
                acc[6].max(chunk[6]),
                acc[7].max(chunk[7]),
                acc[8].max(chunk[8]),
                acc[9].max(chunk[9]),
                acc[10].max(chunk[10]),
                acc[11].max(chunk[11]),
                acc[12].max(chunk[12]),
                acc[13].max(chunk[13]),
                acc[14].max(chunk[14]),
                acc[15].max(chunk[15]),
            ]
        })
    }
}
