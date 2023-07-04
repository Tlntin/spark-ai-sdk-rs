use clap::Parser;
use dialoguer::Password;
use futures::stream::StreamExt;
use spark_ai_sdk_rs::SparkAI;
use std::io::{self, Write};
use tungstenite::error::ProtocolError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    app_id: String,

    #[arg(long)]
    api_key: String,

    /// Warning, do not easily expose private data to the terminal
    #[arg(long, default_value = "")]
    api_secret: String,

    #[arg(long, default_value = "wss://spark-api.xf-yun.com/v1.1/chat")]
    api_url: String,

    /// max_tokens, max is 4096
    #[arg(long, default_value_t = 2048)]
    max_tokens: u32,

    // temperature, default 0.5, range 0.0 ~ 1.0
    #[arg(long, default_value_t = 0.5)]
    temperature: f32,

    // domain, default general
    #[arg(long, default_value = "general")]
    domain: String,

    // user_id, default user1
    #[arg(long, default_value = "user1")]
    user_id: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let app_id = &args.app_id;
    let mut api_secret = args.api_secret;
    let api_key = &args.api_key;
    let api_url = &args.api_url;
    if api_secret == "" {
        api_secret = Password::new()
            .with_prompt("Please input your api secret")
            .interact()
            .unwrap();
    } else {
        println!("Warning, do not easily expose private data to the terminal.");
    }
    let mut spark_ai = SparkAI::new(app_id, api_key, &api_secret, api_url);
    println!("========================================================");
    println!("Welcome to use spark ai chatbot!");
    println!("please input your query, input 'exit' or 'stop' to exit.");
    println!("========================================================");
    loop {
        let query = dialoguer::Input::<String>::new()
            .with_prompt("User")
            .interact()
            .unwrap();
        if query == "exit" || query == "stop" {
            break;
        }

        let mut history = vec![];
        let mut stream = spark_ai
            .chat_stream(
                query.as_str(),
                &mut history,
                &args.user_id,
                &args.domain,
                args.max_tokens,
                args.temperature,
            )
            .await;

        while let Some(result) = stream.next().await {
            match result {
                Ok((response, _history)) => {
                    print!("\rAI: {}", response);
                    io::stdout().flush().unwrap();
                }
                Err(err) => match err {
                    tungstenite::Error::Protocol(err) => {
                        if let ProtocolError::NonZeroReservedBits = err {
                            // if is NonZeroReservedBits type, no print
                        } else {
                            println!("Error: {}", err);
                        }
                    }
                    _ => println!("Error: {}", err),
                },
            }
        }
        println!("");
    }
}
