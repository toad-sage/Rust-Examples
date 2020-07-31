# A Rust Api made with actix-web

## To setup diesel 

1. First Install diesel_cli
2. To set up diesel in project use **`diesel setup`**
3. For every table use **`diesel migration generate <name>`**
4. Write up and down sql
5. run **`diesel migration run`**, and it's ready to use :)


## To start the **Server**

1. install systemfd by **cargo install** and then run **`systemfd --no-pid -s http::8000 -- cargo watch -x run`** in
  terminal. This will start server at **http://127.0.0.1:8000/**
  
  -------------------------------------------------------------------------------------------
  
2. **OR** change the port in *.env* file and hit **`cargo run`**. This will start the server as specified in .env file
