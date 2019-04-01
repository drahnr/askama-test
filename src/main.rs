
#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Debug,PartialEq,Eq)]
pub enum X {
    Recurse (Vec<X>),
    Leaf
}

impl X {
    pub fn is_leaf(&self) -> bool {
        X::Leaf == self
    }

    pub fn iter<'x>(&'x self) -> ChildIter<'x> {
        if let X::Recurse(ref mut inner) = self {
            ChildIter { inner : Some(inner.iter()) }
        } else {
            ChildIter { inner : None }
        }
    }
}



pub struct ChildIter<'x> {
    inner : Option<std::slice::Iter<'x,X>>
}

impl<'x> Iter for ChildIter<'x> {
    type Item = X;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self {
           inner.next()
        } else {
            None
        }
    }
}



#[derive(Template)]
#[template(path = "xyz.html", print = "all")] // using the template in this path, relative
struct Container {
    pub root : Item
}


fn main() {
    println!("try cargo t instead");
}


#[cfg(test)]
mod test {
    #[test]
    fn container() {
        let leaf = Item::Leaf;
        let l2 = Item::Recurse(vec![leaf]);
        let l1 = Item::Recurse(vec![l2]);
        let c = Container { root : l1 };
        println!("{}", c.render().unwrap())
    }
}