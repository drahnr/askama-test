
#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Debug)]
pub enum X<'a> {
    A { name : &'a str } ,
    B (u8),
    C
}



#[derive(Template)]
#[template(path = "q.md", print = "all")] // using the template in this path, relative
struct Container<'a> {
    pub x : X<'a>
}

fn main() {
    println!("try cargo t instead");
}


#[cfg(test)]
mod test {
    #[test]
    fn container() {
        let x = X { name : "content"};
        let c = Container { x };
        println!("{}", c.render().unwrap())
    }
}