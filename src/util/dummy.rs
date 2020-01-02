/// Trait for types that can produce a dummy instance of themselves
pub trait Dummy {
    fn dummy() -> Self;
}
