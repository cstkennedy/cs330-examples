pub trait WrappedType<T>: std::ops::Deref {
    fn inner_value(self) -> T;
}
