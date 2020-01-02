
pub trait Assert<T, E> {
    fn assert(self, p: impl FnOnce(&T) -> bool, err: impl FnOnce() -> E) -> Self;
}

/// Takes a predicate and if the predicate fails (is false), it calls the error generator and returns the result of that
/// The on_err is given as a function and it should only be called when necessary
impl<T, E> Assert<T, E> for Result<T, E> {
    fn assert(self, p: impl FnOnce(&T) -> bool, on_err: impl FnOnce() -> E) -> Self {
        match &self {
            Ok(x)    => if p(x) { self } else { Err(on_err()) },
            Err(_) => self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_assert_on_ok() -> Result<(), ()> {
        let ok = Ok(());
        ok.assert(|_| true, || panic!())
    }

    #[test]
    fn test_failed_assert_on_ok() {
        let ok = Ok(());
        assert_eq!(ok.assert(|_| false, || 5), Err(5))
    }

    #[test]
    fn test_failed_assert_on_err() {
        let ok: Result<(), i32> = Err(0);
        assert_eq!(ok.assert(|_| false, || 5), Err(0))
    }

    #[test]
    fn test_assert_on_err() {
        let ok: Result<(), i32> = Err(0);
        assert_eq!(ok.assert(|_| true, || 5), Err(0))
    }

}
