use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;
use super::{Type, TyScheme, Substitution};

#[derive(Debug, PartialEq)]
pub(crate) struct Env<K, V> where K : Hash + Eq + Debug, V : Debug + PartialEq {
    contexts: Vec<Ctx<K, V>>,
    saved: usize,
}

impl<K> Type for Env<K, TyScheme> where K : Hash + Eq + Debug {
    fn ftv(&self) -> HashSet<u64> {
        self.contexts.iter().map(|t| t.ftv()).fold(HashSet::new(), |acc, x| &acc | &x)
    }

    fn apply(&mut self, _s: &Substitution) { unimplemented!() }
}

impl<'a, K, V> Env<K, V> where K : Hash + Eq + Debug, V : Debug + PartialEq {

    pub fn new() -> Self {
        Self { contexts: vec![Ctx::new()], saved: 1 }
    }

    pub fn define(&mut self, k: K, v: V) { self.contexts.last_mut().unwrap().insert(k, v) }

    /// Saves a scope as a restore point
    pub fn save(&mut self) { self.saved = self.contexts.len() }

    /// Restore pops every scope after the saved one
    /// Saved scope is NOT removed
    pub fn restore(&mut self) { self.contexts.drain(self.saved..); }

    pub fn push(&mut self) { self.contexts.push(Ctx::new()) }

    pub fn pop(&mut self) { self.contexts.pop(); }

    pub fn lookup(&self, k: &K) -> Option<&V> {
        for ctx in self.contexts.iter().rev() {
            if let Some(v) = ctx.lookup(k) { return Some(v) }
        }
        None
    }

}

#[derive(Debug, PartialEq)]
pub(crate) struct Ctx<K, V> where K : Hash + Eq + Debug, V : Debug + PartialEq {
    ctx: HashMap<K, V>
}

impl<K> Type for Ctx<K, TyScheme> where K : Hash + Eq + Debug {
    fn ftv(&self) -> HashSet<u64> {
        self.ctx.values().map(|t| t.ftv()).fold(HashSet::new(), |acc, x| &acc | &x)
    }

    fn apply(&mut self, s: &Substitution) {
        self.ctx.values_mut().for_each(|t| t.apply(s))
    }
}

impl<'a, K, V> Ctx<K, V> where K : Hash + Eq + Debug, V : Debug + PartialEq {

    pub fn new() -> Self {
        Self { ctx: HashMap::new() }
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.ctx.insert(k, v);
    }

    pub fn lookup(&self, k: &K) -> Option<&V> {
        self.ctx.get(k)
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut env = Env::new();
        env.push();
        env.define(0, 0);
        env.push();
        env.save();
        env.define(1, 1);
        env.push();
        env.define(100, 100);
        env.push();
        env.define(200, 200);
        env.restore();
        env.pop();
        env.pop();
        env.save();

        assert_eq!(env, Env::new());
    }
}












