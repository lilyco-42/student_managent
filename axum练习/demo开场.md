## 打算写一个简单的学生管理系统
## 新建项目
```
cargo new sti
```
## 先添加依赖
```
cargo add axum tokio --features tokio/full
```
## 跑一个axum demo 学习一下
```
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```
## 运行
```
cargo run 
打开浏览器访问 http://127.0.0.1:3000
```
