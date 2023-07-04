## English | [中文](https://github.com/Tlntin/spark-ai-sdk-rs/blob/main/README_zh.md)

## what is?
- rust sdk for XunFei (iFLYTEK) Spark AI

## how to use
1. get Spark AI api in https://www.xfyun.cn/

2. install
- build with cargo, then you can find the compiled file `spark-ai-sdk-rs` from the target/release directory.
```bash
cargo build --release
```

3. use
- get help
```bash
./spark-ai-sdk-rs --help
Usage: spark-ai-sdk-rs [OPTIONS] --app-id <APP_ID> --api-key <API_KEY>

Options:
      --app-id <APP_ID>            
      --api-key <API_KEY>          
      --api-secret <API_SECRET>    Warning, do not easily expose private data to the terminal [default: ]
      --api-url <API_URL>          [default: wss://spark-api.xf-yun.com/v1.1/chat]
      --max-tokens <MAX_TOKENS>    max_tokens, max is 4096 [default: 2048]
      --temperature <TEMPERATURE>  [default: 0.5]
      --domain <DOMAIN>            [default: general]
      --user-id <USER_ID>          [default: user1]
  -h, --help                       Print help
  -V, --version                    Print version
```
- sample
```bash
./spark-ai-sdk-rs --app-id xxxx --api-key xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Please input your api secret: [hidden]
========================================================
Welcome to use spark ai chatbot!
please input your query, input 'exit' or 'stop' to exit.
========================================================
User: Hi
AI: Hello! How can I help you today? If you have any questions or need assistance, please feel free to ask.
User: What's your name?
AI: My name is iFLYTEK Spark. My design and construction are carried out by the team of iFLYTEK, and they are constantly updated and improved so that I can provide better services to users.
User: exit
```

