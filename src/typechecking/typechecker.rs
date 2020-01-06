use crate::parsing::{Expr, ExprKind, Binder, Span};
use crate::error::Error;
use super::{TyKind, Ty, Env, Constraint, Type, solve};

pub struct Typechecker<'a> {
    env: Env<&'a str, Ty>
}

impl<'a> Typechecker<'a> {
    pub fn new() -> Self {
        Self { env: Env::new() }
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
            ExprKind::Id { name } => self.env.lookup(&name.as_str())
                .ok_or(Error::new(expr.span, format!("Unbound variable `{}`", name)))
                .map(|ty_ref| (ty_ref.clone(), Constraint::Empty)),
            ExprKind::Let { binder, bound, body } => {
                let Binder { name, ty, .. } = binder;
                self.env.push();
                self.env.define(name, ty.clone());

                let (tbound, cbound) = self.infer(bound)?;
                let c_tbound_eq_binder_annotation = Constraint::Eq(tbound.clone(), ty.clone());

                let (tbody, cbody) = self.infer(body)?;
                let c_binder_eq_bound = Constraint::Eq(ty.clone(), tbound);
                let c_body_eq_let = Constraint::Eq(tbody.clone(), expr.ty.clone());
                let cs = Constraint::conj(vec![
                    c_body_eq_let,
                    c_binder_eq_bound,
                    cbody,
                    cbound,
                    c_tbound_eq_binder_annotation
                ]);
                Ok((tbody, cs))
            }
            ExprKind::Lambda { params, ret, body } => {
                self.env.push();
                let tparams = Ty::new(expr.span, TyKind::Tuple(params.iter().map(|binder| {
                    let Binder { name, ty, .. } = binder;
                    self.env.define(name, ty.clone());
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





