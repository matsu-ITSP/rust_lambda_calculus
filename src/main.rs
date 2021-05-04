fn main() {
    let po = apply(
        Term::Lambda{
            variable: 'x',
            term: Box::new(Term::Value{value: 'x'})}, 
        Term::Value{value: 'y'}
    );
    println!("{}", po.to_string());
}

fn eval(t: Term) -> Term{
    match t{
        Term::Lambda{variable: _, term: _} => t,
        Term::Value{value: _} => t,
        Term::App{term1, term2} => apply(eval(*term1), eval(*term2))
    }
}

fn apply(t1: Term, t2: Term) -> Term{
    match t1{
        Term::Lambda{variable, term} => subst(variable, *term, t2),
        Term::Value{value: _} => Term::App{term1: Box::new(t1), term2: Box::new(t2)},
        Term::App{term1, term2} => apply(eval(*term1), eval(*term2))
    }
}

fn subst(variable: char, term: Term, arg: Term) -> Term{
    match term{
        Term::Lambda{variable, term} => subst(variable, *term, arg), // no multiple variable
        Term::Value{value} => {
            if value == variable{
                arg
            } else {
                Term::Value{value}
            }
        }
        Term::App{term1, term2} => Term::App{
            term1: Box::new(subst(variable, *term1, arg.clone())), 
            term2: Box::new(subst(variable, *term2, arg.clone())), 
        }
    }
}

#[derive(Clone)]
enum Term{
    Lambda {variable: char, term: Box<Term>},
    App {term1: Box<Term>, term2: Box<Term>},
    Value {value: char}
}

impl ToString for Term{
    fn to_string(&self) -> String{
        // TODO
        match self{
            Term::Lambda{variable: _, term: _} => String::from(""),
            Term::Value{value} => value.to_string(),
            Term::App{term1: _, term2: _} => String::from("")
        }
    }
}
/*
impl Clone for Term  {
    fn clone(&self) -> Term {
        match self{
            Term::Lambda{variable, term} => Term::Lambda{
                variable: *variable, 
                term: *term
            },
            Term::Value{value} => Term::Value{value: *value},
            Term::App{term1, term2} => Term::App{
                term1: *term1,
                term2: *term2
            }
        }
    }
}
*/