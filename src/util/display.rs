use crate::language::expr::Expr;

pub fn print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Binary { left, operator, right } => {
            format!("({} {} {})", print_expr(left), operator.lexeme, print_expr(right))
        }
        Expr::Grouping { expression } => print_expr(expression),
        Expr::Literal { value } => format!("{value:?}"),
        Expr::Unary {
            operator,
            right
        } => format!("({} {})", operator.lexeme, print_expr(right))
    }
}