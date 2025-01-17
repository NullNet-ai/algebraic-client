use algebraic_server::AlgebraicClientImpl;

#[derive(Debug)]
pub enum Operation {
    Exponent(u64, u64),
    Factorial(u64),
}

impl Operation {
    pub async fn call_server(&self, client: &AlgebraicClientImpl) -> u64 {
        println!("Calling server from {self:?}");
        let res = match self {
            Operation::Exponent(a, b) => client.exponent(*a, *b).await.unwrap(),
            Operation::Factorial(a) => client.factorial(*a).await.unwrap(),
        };
        println!("Result: {res}");
        res
    }
}
