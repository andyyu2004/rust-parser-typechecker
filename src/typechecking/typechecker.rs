use crate::parsing::{Expr, ExprKind, Binder};
use crate::error::Error;
use super::{Ty, Env, Constraint, Type, solve};

pub struct Typechecker<'a> {
    env: Env<&'a str, Ty>
}

impl<'a> Typechecker<'a> {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn typecheck(&mut self, expr: &'a Expr<'a>) -> Result<Ty, Vec<Error>> {
        let (mut t, c) = self.infer(expr).map_err(|e| vec![e])?;
        println!("c: {}", c);
        let substitution = solve(c).map_err(|err| vec![err])?;
        t.apply(&substitution);
        Ok(t)
    }

    pub fn infer(&mut self, expr: &'a Expr<'a>) -> Result<(Ty, Constraint), Error> {
        match &expr.kind {
            ExprKind::Bool { .. } | ExprKind::Integral { .. } => Ok(self.typecheck_literal(expr)),
            ExprKind::Id { name } => self.env.lookup(&name.lexeme)
                .ok_or(Error::new(*name, format!("Unbound variable")))
                .map(|ty_ref| (ty_ref.clone(), Constraint::Empty)),
            ExprKind::Let { binder, bound, body } => {
                let Binder { name, ty } = binder;
                self.env.push();
                self.env.define(name.lexeme, ty.clone());

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
                let tparams = Ty::Tuple(params.iter().map(|binder| {
                    let Binder { name, ty } = binder;
                    self.env.define(name.lexeme, ty.clone());
                    binder.ty.clone()
                }).collect::<Vec<_>>());

                let (tbody, cbody) = self.infer(body)?;
                let tlambda = Ty::Arrow(box tparams, box tbody.clone());
                let clambda = Constraint::Eq(tlambda.clone(), expr.ty.clone());
                let c_ret_eq_body = Constraint::Eq(tbody, ret.clone());
                let cs = Constraint::conj(vec![clambda, c_ret_eq_body, cbody]);

                self.env.pop();
                Ok((tlambda, cs))
            }
            ExprKind::App { f, args } => {
                let (tf, cf) = self.infer(f)?;
                let xs = args.iter().map(|e| self.infer(e)).collect::<Result<Vec<_>, _>>()?;
                let (targs, mut cargs) = crate::util::split(xs);
                let capp = Constraint::Eq(tf, Ty::Arrow(box Ty::Tuple(targs), box expr.ty.clone()));
                cargs.extend(vec![cf, capp]);
                let cs = Constraint::conj(cargs);
                Ok((expr.ty.clone(), cs))
            }
            ExprKind::Block { exprs, suppressed } => {
                if *suppressed {
                    for expr in exprs { self.infer(expr)?; }
                    Ok((Ty::Tuple(vec![]), Constraint::Empty))
                } else {
                    // If a block is unsuppressed, it must have at least one expression
                    for expr in &exprs[..exprs.len() - 1] { self.infer(expr)?; }
                    let last_expr = exprs.last().unwrap();
                    self.infer(last_expr)
                }
            }
            _ => unimplemented!("{}", expr),
        }
    }

    fn typecheck_literal(&self, expr: &'a Expr<'a>) -> (Ty, Constraint) {
        debug_assert_eq!(Self::typeof_literal(&expr.kind), expr.ty);
        (expr.ty.clone(), Constraint::Empty)
    }

    fn typeof_literal(kind: &ExprKind<'a>) -> Ty {
        match kind {
            ExprKind::Bool { .. }     => Ty::Bool,
            ExprKind::Integral { .. } => Ty::I64,
            _ => panic!("Not a literal")
        }
    }
}
