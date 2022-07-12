use crate::expr::Expr;

pub trait AstPrinter {
    fn accept(&self) -> String;
}

impl AstPrinter for Expr {
    fn accept(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => parenthesize(&operator.lexeme, &vec![left, right]),
            Expr::Grouping { expression } => parenthesize("group", &vec![expression]),
            Expr::Literal { value } => value.to_string(),
            Expr::Unary { operator, right } => parenthesize(&operator.lexeme, &vec![right]),
            Expr::Assign { name, value } => todo!(),
            Expr::Variable { name } => todo!(),
        }
    }
}

fn parenthesize(name: &str, exprs: &Vec<&Expr>) -> String {
    let mut sb = vec!["(".to_string(), name.to_string()];

    for expr in exprs {
        sb.push(" ".to_string());
        sb.push(expr.accept());
    }
    sb.push(")".to_string());

    sb.join("")
}
