use resp::{parse_command, ResponseHandler, Value};
use tokio::net::{TcpListener, TcpStream};

mod commands;
mod resp;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((stream, _)) => {
                tokio::spawn(async move { handle_connection(stream).await });
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream) {
    let mut handler = ResponseHandler::new(stream);

    loop {
        let value = handler.read_value().await.unwrap();

        let response = if let Some(val) = value {
            match parse_command(val) {
                Ok((command, args)) => match command.to_lowercase().as_str() {
                    "quit" => commands::quit(),
                    "info" => commands::info(),
                    "ping" => commands::ping(args),
                    unknown => Value::SimpleError(format!("Unknown command '{}'", unknown)),
                },
                _ => {
                    break;
                }
            }
        } else {
            break;
        };

        handler.write_value(response).await.unwrap();
    }
}
