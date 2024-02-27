use std::fmt;

const DEFAULT_NAME: &'static str = "John Q. Smith";

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Student {
    pub name: &'static str,
}

impl Student {
    pub fn new(nme: &'static str) -> Self {
        Student {
            name: nme,
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
