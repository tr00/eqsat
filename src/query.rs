/** grammar of rewrite patterns
 * 
 * patlist ::= pattern '\n'+ patlist | pattern
 * pattern ::= expr === expr
 * 
 * expr ::= kind '(' varlist ')' | symbol
 *
 */

#[derive(Debug)]
struct PatternExpression {
    
}

// R(x1, x2, ..., xk)

#[derive(Debug)]
struct Relation {
    kind: i32,
    vars: Vec<i32>,
}


// Q(x1, x2, x3) <-- R(x1, x2) & R(x2, x3) & R(x1, x3)

#[derive(Debug)]
struct ConjunctiveQuery {
    free_vars: Vec<i32>,
    bound_vars: Vec<i32>,
    relations: Vec<Relation>,
}
