use crate::parsing::{Expr, ExprKind, Binder, Span};
use crate::error::Error;
use super::{TyKind, Ty, Env, Constraint, Type, TyScheme, solve};
use crate::util::{self, Counter};
use std::mem;

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
                    binder.ty.take()
                }).collect::<Vec<_>>()));

                let (tbody, cbody) = self.infer(body)?;
                let tlambda = Ty::new(expr.span, TyKind::Arrow(box tparams, box tbody.clone()));
                let clambda = Constraint::Eq(tlambda.clone(), expr.ty.take());
                let c_ret_eq_body = Constraint::Eq(tbody, ret.take());
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
                let capp = Constraint::Eq(tf, Ty::new(fspan, TyKind::Arrow(targs, box expr.ty.take())));
                cargs.extend(vec![cf, capp]);
                let cs = Constraint::conj(cargs);
                Ok((expr.ty.clone(), cs))
            }
            ExprKind::Block { exprs, suppressed } => {
                self.env.save();
                self.env.push();
                let xs = exprs.iter_mut().map(|e| self.infer(e)).collect::<Result<Vec<_>,_>>()?;
                let (mut types, constraints) = util::split(xs);
                let block_type = if *suppressed { Ty::new(expr.span, TyKind::unit()) } else { types.remove(types.len() - 1) };
                self.env.restore();
                Ok((block_type, Constraint::conj(constraints)))
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





