use crate::parsing::{Expr, ExprKind, Span};
use crate::error::Error;
use super::{TyKind, Ty, Env, Constraint, Type, TyScheme, solve};
use crate::util::{self, Counter};

pub struct Typechecker<'a> {
    env: Env<&'a str, TyScheme>,
    name_gen: &'a mut Counter,
}

impl<'a> Typechecker<'a> {
    pub fn new(name_gen: &'a mut Counter) -> Self {
        Self { env: Env::new(), name_gen }
    }

    pub fn typecheck(&mut self, expr: &'a mut Expr) -> Result<Ty, Vec<Error>> {
        let (mut t, c) = self.infer(expr).map_err(|e| vec![e])?;
        println!("c: {}", c);
        let substitution = solve(c).map_err(|err| vec![err])?;
        t.apply(&substitution);
        Normalizer::new().normalize(&mut t);
        Ok(t)
    }

    pub fn infer(&mut self, expr: &'a mut Expr) -> Result<(Ty, Constraint), Error> {
        match &mut expr.kind {
            ExprKind::Id { name } => {
                let scheme = self.env.lookup(&name.as_str())
                    .ok_or(Error::new(expr.span, format!("Unbound variable `{}`", name)))?;
                Ok((scheme.instantiate(self.name_gen), Constraint::Empty))
            }
            ExprKind::Let { binder, bound } => {
                self.env.push();
                let (tbound, cbound) = self.infer(bound)?;
                let c_tbound_eq_binder_annotation = box Constraint::Eq(tbound.clone(), binder.ty.clone());
                let c = Constraint::And(box cbound, c_tbound_eq_binder_annotation);
                let s = solve(c.clone())?;
                let mut principle_ty = tbound.clone();
                principle_ty.apply(&s);
                let generalized = principle_ty.generalize(&self.env);
                self.env.define(&binder.name, generalized);
                let tret = Ty::new(expr.span, TyKind::unit()); // Let expressions always return unit;
                Ok((tret, c))
            }
            ExprKind::Lambda { params, ret, body } => {
                self.env.push();
                let tparams = Ty::new(expr.span, TyKind::Tuple(params.iter_mut().map(|binder| {
                    self.env.define(&binder.name, TyScheme::from(binder.ty.clone()));
                    binder.ty.clone()
                }).collect::<Vec<_>>()));

                let (tbody, cbody) = self.infer(body)?;
                let tlambda = Ty::new(expr.span, TyKind::Arrow(box tparams, box tbody.clone()));
                let clambda = Constraint::Eq(tlambda.clone(), expr.ty.clone());
                let c_ret_eq_body = Constraint::Eq(tbody, ret.clone());
                let cs = Constraint::conj(vec![clambda, c_ret_eq_body, cbody]);

                self.env.pop();
                Ok((tlambda, cs))
            }
            ExprKind::App { f, args } => {
                let fspan = f.span; // for borrow checker reasons
                let (tf, cf) = self.infer(f)?;
                let xs = args.iter_mut().map(|e| self.infer(e)).collect::<Result<Vec<_>, _>>()?;
                let (vargs, mut cargs) = util::split(xs);
                let targs = box Ty::new(expr.span, TyKind::Tuple(vargs));
                let capp = Constraint::Eq(tf, Ty::new(fspan, TyKind::Arrow(targs, box expr.ty.clone())));
                cargs.extend(vec![cf, capp]);
                let cs = Constraint::conj(cargs);
                Ok((expr.ty.clone(), cs))
            }
            ExprKind::Block { exprs, suppressed } => {
                self.env.save();
                self.env.push();
                let xs = exprs.iter_mut().map(|e| self.infer(e)).collect::<Result<Vec<_>, _>>()?;
                let (mut types, constraints) = util::split(xs);
                let block_type = if *suppressed { Ty::new(expr.span, TyKind::unit()) } else { types.remove(types.len() - 1) };
                self.env.restore();
                Ok((block_type, Constraint::conj(constraints)))
            }
            ExprKind::Tuple { elems } => {
                let xs = elems.iter_mut().map(|e| self.infer(e)).collect::<Result<Vec<_>, _>>()?;
                let (types, constraints) = util::split(xs);
                let ty = Ty::new(expr.span, TyKind::Tuple(types));
                Ok((ty, Constraint::conj(constraints)))
            }
            ExprKind::Grouping { expr } => self.infer(expr),
            k@ExprKind::Bool { .. } | k@ExprKind::Integral { .. } => Ok(Self::typecheck_literal(k, &expr.ty, expr.span)),
            expr => unimplemented!("{}", expr),
        }
    }

    fn typecheck_literal(exprkind: &ExprKind, ty: &Ty, span: Span) -> (Ty, Constraint) {
        debug_assert_eq!(ty.kind, Self::type_of_literal_expr(exprkind));
        (Ty::new(span, Self::type_of_literal_expr(exprkind)), Constraint::Empty)
    }

    fn type_of_literal_expr(exprkind: &ExprKind) -> TyKind {
        match exprkind {
            ExprKind::Integral { .. } => TyKind::I64,
            ExprKind::Bool { .. }     => TyKind::Bool,
            _ => panic!("{} is not a literal", exprkind)
        }
    }

}

use variable_gen::Generator;
use std::collections::HashMap;

/// simplfiies type names
struct Normalizer {
    name_gen: Generator,
    names: HashMap<u64, String>,
}

impl Normalizer {

    pub fn new() -> Self {
        Self { name_gen: Generator::new(), names: HashMap::new() }
    }

    pub fn normalize(&mut self, ty: &mut Ty) {
        match &mut ty.kind {
            TyKind::Infer(i) => match self.names.get(i) {
                Some(name) => ty.kind = TyKind::TyVar(name.clone()),
                None => {
                    let new_name = self.name_gen.gen();
                    self.names.insert(*i, new_name.clone());
                    ty.kind = TyKind::TyVar(new_name);
                }
            }
            TyKind::Arrow(box l, box r) => {
                self.normalize(l);
                self.normalize(r);
            }
            TyKind::Tuple(ts) => ts.iter_mut().for_each(|t| self.normalize(t)),
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::arrow;

    macro_rules! typecheck { ($src:expr) => { { crate::generate_ast($src).unwrap().0 } } }

    #[test] fn typeof_int() { assert_eq!(typecheck!("5"), TyKind::I64.to_ty()) }
    #[test] fn typeof_bool() { assert_eq!(typecheck!("false"), TyKind::Bool.to_ty()) }

    #[test]
    fn typeof_tuple() {
        assert_eq!(
            typecheck!("(1, false)"),
            // (i64, bool)
            TyKind::Tuple(vec![
                TyKind::I64.to_ty(),
                TyKind::Bool.to_ty(),
            ]).to_ty()
        )
    }

    #[test]
    fn typeof_id_fn() {
        assert_eq!(
            typecheck!("fn x: Int => x"),
            // (i64) -> i64
            arrow!(TyKind::I64.to_ty().singleton() => TyKind::I64.to_ty())
        )
    }

    #[test]
    fn typeof_simple_fn() {
        assert_eq!(
            typecheck!("fn x: Int => false"),
            // (i64) -> bool
            arrow!(TyKind::I64.to_ty().singleton() => TyKind::Bool.to_ty())
        )
    }

    #[test]
    fn typeof_uncurry() {
        let t = typecheck!("fn x => fn y => (x, y)");
        // (a) -> (b) -> (a, b)
        let expected = arrow!(
            TyKind::TyVar("a".to_owned()).to_ty().singleton() =>
            TyKind::TyVar("b".to_owned()).to_ty().singleton() =>
            TyKind::Tuple(vec![
                TyKind::TyVar("a".to_owned()).to_ty(),
                TyKind::TyVar("b".to_owned()).to_ty(),
            ]).to_ty()
        );
        assert_eq!(t, expected);
    }

    #[test]
    fn typeof_double_application() {
        let t = typecheck!("fn f => fn x => f(f(x))");
        // ((a) -> a) -> (a) -> a
        let tf = arrow!(TyKind::TyVar("a".to_owned()).to_ty().singleton() => TyKind::TyVar("a".to_owned()).to_ty()).singleton();
        let expected = arrow!(tf => TyKind::TyVar("a".to_owned()).to_ty().singleton() => TyKind::TyVar("a".to_owned()).to_ty());
        assert_eq!(t, expected)
    }


}













