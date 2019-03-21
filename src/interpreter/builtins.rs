use crate::parser::ast::*;
use crate::interpreter::message::*;
use rand::Rng;

fn exprvec_to_vec(vec: &[Expr]) -> Vec<String> {
    vec.iter().filter_map(|elem|
        match elem {
           Expr::LitExpr(Literal::StringLiteral(string))    => Some(string.clone()),
           Expr::LitExpr(Literal::IntLiteral(int))          => Some(int.to_string()),
           _                                                => None
        }
    ).collect::<Vec<String>>()
}

// return Result<Expr, error>
pub fn typing(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(Literal::IntLiteral(_)) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn wait(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(Literal::IntLiteral(_)) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn text(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn button(args: &Expr) -> MessageType {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 2 {
            if let (Expr::LitExpr(Literal::StringLiteral(arg1)), Expr::VecExpr(arg2)) = (&vec[0], &vec[1]) {
                return MessageType::Msg( Message {
                    my_type: "Button".to_string(),
                    content: Content::Button(arg1.to_string(), exprvec_to_vec(arg2))
                })
            }
        }
    }
    MessageType::Msg( Message {my_type: "say".to_owned(), content: Content::Button("Button".to_owned(), vec![])})
}

// return Result<Expr, error>
pub fn url(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        if vec.len() == 1 {
            if let Expr::LitExpr(_) = &vec[0] {
                return &vec[0];
            }
        }
        return &vec[0];
    }
    args
}

// return Result<Expr, error>
pub fn one_of(args: &Expr) -> &Expr {
    if let Expr::VecExpr(vec) = args {
        return &vec[rand::thread_rng().gen_range(0, vec.len())];
    }
    args
}