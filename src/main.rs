mod operation;

use crate::operation::Operation;
use algebraic_server::AlgebraicGrpcInterface;
use file_monitor::FileMonitor;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[tokio::main]
async fn main() {
    let mut client = AlgebraicGrpcInterface::new("localhost", 50051).await;

    let config_pair = Arc::new((Mutex::new(String::new()), Condvar::new()));
    let config_pair_2 = config_pair.clone();
    let file_monitor = FileMonitor::new(
        "../algebraic-client/input/input.txt".to_string(),
        config_pair_2,
    );

    thread::spawn(move || {
        file_monitor.monitor();
    });

    loop {
        drop(config_pair.1.wait(config_pair.0.lock().unwrap()).unwrap());
        let file = config_pair.0.lock().unwrap().clone();
        let lines = file.lines();
        for line in lines {
            let Some(operation) = parse_line(line) else {
                panic!("Invalid operation: {line}");
            };
            operation.call_server(&mut client).await;
        }
    }
}

fn parse_line(line: &str) -> Option<Operation> {
    let (op, rest) = line.split_once(' ')?;
    match op {
        "pow" => {
            let (a, b) = rest.split_once(',')?;
            Some(Operation::Exponent(
                a.parse::<f32>().ok()?,
                b.parse::<u32>().ok()?,
            ))
        }
        "factorial" => Some(Operation::Factorial(rest.parse::<u32>().ok()?)),
        _ => None,
    }
}
