pub struct DeltaList<const N: usize> {
    list: [DeltaItem; N],
}

impl<const N: usize> DeltaList<N> {
    pub const fn new() -> Self {
        DeltaList {
            list: [const { DeltaItem::new() }; N],
        }
    }
}

struct DeltaItem {
    id: usize,
    delta: usize,
}

impl DeltaItem {
    pub const fn new() -> Self {
        DeltaItem { id: 0, delta: 0 }
    }
}
