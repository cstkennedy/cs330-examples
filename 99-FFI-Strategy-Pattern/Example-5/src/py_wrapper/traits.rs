pub mod language {
    pub struct PythonForRust;
    pub struct RustForPython;
}

pub trait DunderRepr {
    type Error;

    type WrappedLanguage;

    fn repr_helper(&self) -> Result<String, Self::Error>;
}

impl<T> DunderRepr for Option<T>
where T: DunderRepr
{
    type Error = <T as DunderRepr>::Error;

    type WrappedLanguage = <T as DunderRepr>::WrappedLanguage;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        match self {
            Some(thing) => thing.repr_helper(),
            None => Ok("...".to_string())
        }
    }
}
