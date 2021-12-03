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

    fn may_apply(&self, e: &Exp) -> Option<Exp> {
        match e {
            App(e1, e2) => {
                let mut did = false;
                let n = App(
                    {
                        if let Some(e1) = self.may_apply(*e1) {
                            did = true;
                            Box::new(e1)
                        } else {
                            e1
                        }
                    },
                    {
                        if let Some(e2) = self.may_apply(*e2) {
                            did = true;
                            Box::new(e2)
                        } else {
                            e2
                        }
                    },
                );
                if did {
                    Some(n)
                } else {
                    None
                }
            }
            Lam(x, e_) if !self.0.contains_key(&x) => {
                if let Some(e) = self.may_apply(*e_) {
                    Some(Lam(x, Box::new(e)))
                } else {
                    None
                }
            }
            Let(x, e1, e2) if !self.0.contains_key(&x) => {
                let mut did = false;
                let n = Let(
                    x,
                    {
                        if let Some(e1) = self.may_apply(*e1) {
                            did = true;
                            Box::new(e1)
                        } else {
                            e1
                        }
                    },
                    {
                        if let Some(e2) = self.may_apply(*e2) {
                            did = true;
                            Box::new(e2)
                        } else {
                            e2
                        }
                    },
                );
                if did {
                    Some(n)
                } else {
                    None
                }
            }
            Var(x) => {
                if let Some(e) = self.0.get(&x) {
                    Some(e.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
