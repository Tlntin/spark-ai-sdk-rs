## [English](README.md) | 中文

## 这是什么？
- 讯飞星火AI大模型的Rust SDK

## how to use
1. 需要从官网 https://www.xfyun.cn/ 获取API

2. 安装
- 通过Cargo编译，然后可以从target/release目录找到编译好的文件`spark-ai-sdk-rs`
```bash
cargo build --release
```

3. 使用
- 获取帮助
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

- 使用案例
```bash
./spark-ai-sdk-rs --app-id xxxx --api-key xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Please input your api secret: [hidden]
========================================================
Welcome to use spark ai chatbot!
please input your query, input 'exit' or 'stop' to exit.
========================================================
User: 你好
AI: 你好！有什么我可以帮助你的吗？
User: 你的名字?
AI: 您好，我是科大讯飞研发的认知智能大模型，我的名字叫讯飞星火认知大模型。我可以和人类进行自然交流，解答问题，高效完成各领域认知智能需求。
User: exit
```