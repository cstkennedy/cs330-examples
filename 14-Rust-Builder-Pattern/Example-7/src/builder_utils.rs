#[derive(Default, Clone, Debug, PartialEq)]
pub struct WrappedType<T>(T);

impl<T> WrappedType<T> {
    pub fn inner_value(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for WrappedType<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for WrappedType<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for WrappedType<T> {
    fn from(value: T) -> Self {
        WrappedType::<T>(value)
    }
}

/*
impl<T> From<WrappedType<T>> for T {
    fn from(value: WrappedType<T>) -> Self {
        value.0
    }
}
*/

impl From<&str> for WrappedType<String> {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}
