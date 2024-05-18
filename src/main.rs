use std::{env, sync::Arc};

use database::Database;
use resp::{parse_command, ResponseHandler, Value};
use tokio::net::{TcpListener, TcpStream};

mod args;
mod commands;
mod database;
mod resp;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port = args::get(&args, "--port").unwrap_or(6379);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    let db = Arc::new(Database::new());

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((stream, _)) => {
                let _db = Arc::clone(&db);
                tokio::spawn(async move { handle_connection(stream, _db).await });
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream, db: Arc<Database>) {
    let mut handler = ResponseHandler::new(stream);

    loop {
        let value = handler.read_value().await;

        if let Ok(value) = value {
            let response = if let Some(val) = value {
                match parse_command(val) {
                    Ok((command, args)) => match command.to_lowercase().as_str() {
                        "quit" => commands::quit(),
                        "info" => commands::info(),
                        "ping" => commands::ping(args),
                        "echo" => commands::echo(args),
                        "get" => commands::get(args, &db).await,
                        "set" => commands::set(args, &db).await,
                        "del" => commands::del(args, &db).await,
                        "ttl" => commands::ttl(args, &db).await,
                        "expire" => commands::expire(args, &db).await,
                        unknown => Value::SimpleError(format!("ERR Unknown command '{}'", unknown)),
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
}
