pub trait Character: core::fmt::Debug + Send + Sync {
    fn hit_point(&self) -> usize;
}
