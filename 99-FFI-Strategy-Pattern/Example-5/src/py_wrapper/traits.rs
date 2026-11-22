pub trait DunderRepr {
    type Error;

    fn repr_helper(&self) -> Result<String, Self::Error>;
}
