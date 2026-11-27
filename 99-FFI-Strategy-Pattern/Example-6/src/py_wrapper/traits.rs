pub mod language {
    pub struct PythonForRust;
    pub struct RustForPython;
}

pub trait PyWrapped {
    type Language;
    type WrappedBy;
}

pub trait PyWrapper {
    type WrappedType;

    fn get_wrapped(&self) -> &Self::WrappedType;
}

pub trait DunderRepr {
    type Error;

    fn repr_helper(&self) -> Result<String, Self::Error>;
}

impl<T> DunderRepr for Option<T>
where
    T: DunderRepr,
{
    type Error = <T as DunderRepr>::Error;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        match self {
            Some(thing) => thing.repr_helper(),
            None => Ok("...".to_string()),
        }
    }
}

impl<T> DunderRepr for T
where
    T: PyWrapper,
    T::WrappedType: DunderRepr + PyWrapped<Language = language::RustForPython>,
{
    type Error = <T::WrappedType as DunderRepr>::Error;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        self.get_wrapped().repr_helper()
    }
}
