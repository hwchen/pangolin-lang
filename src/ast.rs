pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self); // dummy fn
}

pub trait Expression: Node {
    fn expression_node(&self); // dummy fn
}

pub struct Program {
    statements: Vec<Box<Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        let res: Vec<_> = self.statements.iter()
            .map(|statement| statement.token_literal())
            .collect();

        res.join(" ")
    }
}
