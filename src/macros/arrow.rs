
#[macro_export]
macro_rules! arrow {
    ( ($e:expr $( => $es:expr )* ) ) => { arrow!( $e $( => $es )*) }; // pattern to match redundant parens (i.e. nothing follows it)
    ( ($e:expr $( => $es:expr )* ) $( => $rest:expr )+ ) => {
        crate::typechecking::TyKind::Arrow(
            box arrow!($e $( => $es )* ),
            box arrow!( $( $rest ) => + )
        ).to_ty()
    };
    ( $e:expr ) => { $e };
    ( $e:expr $( => $es:expr )+ ) => { crate::typechecking::TyKind::Arrow(box $e, box arrow!( $( $es ) => + ) ).to_ty() };
}

#[cfg(test)]
mod test {
    use crate::typechecking::*;

    #[test]
    fn single_arrow() {
        let t = arrow!(TyKind::F64.to_ty());
        assert_eq!(t, TyKind::F64.to_ty())
    }

    #[test]
    fn double_arrow() {
        let t = arrow!(TyKind::F64.to_ty() => TyKind::Bool.to_ty());
        assert_eq!(t, TyKind::Arrow(box TyKind::F64.to_ty(), box TyKind::Bool.to_ty()).to_ty())
    }

    #[test]
    fn redundant_paren_arrow() {
        let t = arrow!( (TyKind::F64.to_ty() => TyKind::Bool.to_ty()) );
        let expected = TyKind::Arrow(
            box TyKind::F64.to_ty(),
            box TyKind::Bool.to_ty(),
        ).to_ty();
        assert_eq!(t, expected);
    }

    #[test]
    fn paren_arrow() {
        let t = arrow!((TyKind::Bool.to_ty() => TyKind::F64.to_ty()) => TyKind::I64.to_ty());
        let expected = TyKind::Arrow(
            box TyKind::Arrow(
                box TyKind::Bool.to_ty(),
                box TyKind::F64.to_ty()
            ).to_ty(),
            box TyKind::I64.to_ty(),
        ).to_ty();
        assert_eq!(t, expected);
    }

    #[test]
    fn multi_arrow() {
        let t0 = TyKind::F64.to_ty();
        let t1 = TyKind::I64.to_ty();
        let t2 = TyKind::Bool.to_ty();
        let t = arrow!(t0 => t1 => t2);
        let expected = TyKind::Arrow(
            box TyKind::F64.to_ty(),
            box TyKind::Arrow(
                box TyKind::I64.to_ty(),
                box TyKind::Bool.to_ty()
            ).to_ty()
        ).to_ty();
        assert_eq!(t, expected);
    }
}


