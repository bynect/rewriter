use crate::{Exp, Exp::*};
use immutable_map::TreeMap;

#[derive(Clone)]
pub struct Subst<T: Clone>(TreeMap<String, T>);

impl Subst<Exp> {
    pub fn new() -> Self {
        Subst(TreeMap::new())
    }

    pub fn extend(&self, k: String, v: Exp) -> Self {
        Subst(self.0.insert(k, v))
    }

    pub fn apply(&self, e: Exp) -> Exp {
        match e {
            App(e1, e2) => App(Box::new(self.apply(*e1)), Box::new(self.apply(*e2))),
            Lam(x, e) if !self.0.contains_key(&x) => Lam(x, Box::new(self.apply(*e))),
            Let(x, e1, e2) if !self.0.contains_key(&x) => {
                Let(x, Box::new(self.apply(*e1)), Box::new(self.apply(*e2)))
            }
            Var(x) => {
                if let Some(e) = self.0.get(&x) {
                    e.clone()
                } else {
                    Var(x)
                }
            }
            _ => e,
        }
    }

    pub fn contain(&self, k: &String) -> bool {
        self.0.contains_key(k)
    }
}
