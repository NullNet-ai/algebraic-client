use algebraic_server::{AlgebraicGrpcInterface, ExponentMessage, FactorialMessage};

#[derive(Debug)]
pub enum Operation {
    Exponent(f32, u32),
    Factorial(u32),
}

impl Operation {
    pub async fn call_server(&self, client: &mut AlgebraicGrpcInterface) -> f32 {
        println!("Calling server with {self:?}");
        let res = match self {
            Operation::Exponent(a, b) => {
                {
                    let message = ExponentMessage {
                        base: *a,
                        exponent: *b,
                    };
                    client.exponent(message).await.unwrap()
                }
                .value
            }
            Operation::Factorial(a) => {
                let message = FactorialMessage { value: *a };
                #[allow(clippy::cast_precision_loss)]
                let val = client.factorial(message).await.unwrap().value as f32;
                val
            }
        };
        println!("Result: {res}");
        res
    }
}
