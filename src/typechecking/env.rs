use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub(crate) struct Env<K, V> where K : Hash + Eq + Debug, V : Debug + PartialEq {
    contexts: Vec<Ctx<K, V>>,
    saved: usize,
}

impl<'a, K, V> Env<K, V> where K : Hash + Eq + Debug, V : Debug + PartialEq {

    pub fn new() -> Self {
        Self { contexts: vec![Ctx::new()], saved: 1 }
    }

    pub fn define(&mut self, k: K, v: V) {
        self.contexts.last_mut().unwrap().insert(k, v);
    }

    /// Saves a scope as a restore point
    pub fn save(&mut self) { self.saved = self.contexts.len() }

    /// Restore pops every scope after the saved one
    /// Saved scope is NOT removed
    pub fn restore(&mut self) {
        self.contexts.drain(self.saved..);
    }

    pub fn push(&mut self) {
        self.contexts.push(Ctx::new())
    }

    pub fn pop(&mut self) {
        self.contexts.pop();
    }

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












