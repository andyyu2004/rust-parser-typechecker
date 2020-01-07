use crate::parsing::{Expr, ExprKind, Binder, Span};
use crate::error::Error;
use super::{TyKind, Ty, Env, Constraint, Type, TyScheme, solve};
use crate::util::Counter;

pub struct Typechecker<'a> {
    env: Env<&'a str, TyScheme>,
    name_gen: &'a mut Counter,
}

impl<'a> Typechecker<'a> {
    pub fn new(name_gen: &'a mut Counter) -> Self {
        Self { env: Env::new(), name_gen }
    }

    pub fn typecheck(&mut self, expr: &'a Expr) -> Result<Ty, Vec<Error>> {
        let (mut t, c) = self.infer(expr).map_err(|e| vec![e])?;
        println!("c: {}", c);
        let substitution = solve(c).map_err(|err| vec![err])?;
        t.apply(&substitution);
        Ok(t)
    }

    pub fn infer(&mut self, expr: &'a Expr) -> Result<(Ty, Constraint), Error> {
        match &expr.kind {
            ExprKind::Bool { .. } | ExprKind::Integral { .. } => Ok(self.typecheck_literal(expr)),
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
                let tparams = Ty::new(expr.span, TyKind::Tuple(params.iter().map(|binder| {
                    let Binder { name, ty, .. } = binder;
                    self.env.define(name, TyScheme::from(ty));
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
                let (tf, cf) = self.infer(f)?;
                let xs = args.iter().map(|e| self.infer(e)).collect::<Result<Vec<_>, _>>()?;
                let (vargs, mut cargs) = crate::util::split(xs);
                let targs = box Ty::new(expr.span, TyKind::Tuple(vargs));
                let capp = Constraint::Eq(tf, Ty::new(f.span, TyKind::Arrow(targs, box expr.ty.clone())));
                cargs.extend(vec![cf, capp]);
                let cs = Constraint::conj(cargs);
                Ok((expr.ty.clone(), cs))
            }
            ExprKind::Block { exprs, suppressed } => {
                self.env.save();
                self.env.push();
                let xs = exprs.iter().map(|e| self.infer(e)).collect::<Result<Vec<_>,_>>()?;
                let block_type = if *suppressed { Ty::new(expr.span, TyKind::Tuple(vec![])) } else { xs.last().unwrap().0.clone() };
                let constraints = xs.into_iter().map(|(_, c)| c).collect::<Vec<_>>();
                self.env.restore();
                Ok((block_type, Constraint::conj(constraints)))
            }
            ExprKind::Grouping { expr } => self.infer(expr),
            _ => unimplemented!("{}", expr),
        }
    }

    fn typecheck_literal(&self, expr: &'a Expr) -> (Ty, Constraint) {
        debug_assert_eq!(Self::typeof_literal(expr), expr.ty);
        (expr.ty.clone(), Constraint::Empty)
    }

    fn typeof_literal(expr: &Expr) -> Ty {
        let kind = match expr.kind {
            ExprKind::Bool { .. }     => TyKind::Bool,
            ExprKind::Integral { .. } => TyKind::I64,
            _ => panic!("Not a literal")
        };
        Ty::new(expr.span, kind)
    }
}





