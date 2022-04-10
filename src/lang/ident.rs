#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    name: String,
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
