use crate::{Expr, Expr::*};
use immutable_map::TreeMap;

#[derive(Clone)]
pub struct Subst<T: Clone>(TreeMap<String, T>);

impl Subst<Expr> {
    pub fn new() -> Self {
        Subst(TreeMap::new())
    }

    pub fn extend(&self, k: String, v: Expr) -> Self {
        if self.0.is_empty() {
            Subst(self.0.insert(k, v))
        } else {
            let s1 = Subst(TreeMap::new().insert(k.clone(), v.clone()));
            let mut s2 = TreeMap::<String, Expr>::new();
            for (k, v) in self.0.iter() {
                s2 = s2.insert(k.clone(), s1.apply(v.clone()));
            }
            Subst(s2.insert(k, v))
        }
    }

    pub fn apply(&self, e: Expr) -> Expr {
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
}
