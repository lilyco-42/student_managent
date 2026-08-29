use axum::extract::State;
use axum::response::Json;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
#[tokio::main]
async fn main() {
    let (mut 小红, mut 小明, mut 小王) = (学生::new(), 学生::new(), 学生::new());

    小红.设置姓名("小工".to_string());
    小明
        .设置姓名("小明".to_string())
        .设置年龄(19)
        .添加学科(vec!["数学".to_string(), "英语".to_string()]);

    小王
        .设置姓名("小王".to_string())
        .添加学科(vec!["语文".to_string(), "数学".to_string()])
        .添加学科(vec!["数学".to_string(), "英语".to_string()]); // "数学" 不会重复

    let 学生列表 = vec![小红, 小明, 小王];
    // dbg!(&学生列表);
    let 监听器 = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let 路由 = axum::Router::new()
        .route("/", get(获取学生列表))
        .with_state(学生列表);

    axum::serve(监听器, 路由).await.unwrap();
}
// 处理函数：通过 State 提取共享的学生列表，返回 JSON
//

async fn 获取学生列表(State(数据): State<Vec<学生>>) -> Json<Vec<学生>> {
    Json(数据)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct 学生 {
    姓名: String,
    年龄: u8,
    学科: Vec<String>,
    分数: Vec<u32>,
    成绩: HashMap<String, u32>,
}

impl 学生 {
    fn new() -> Self {
        Self::default()
    }

    pub fn 设置姓名(&mut self, 姓名: String) -> &mut Self {
        self.姓名 = 姓名;
        self
    }

    pub fn 设置年龄(&mut self, 年龄: u8) -> &mut Self {
        self.年龄 = 年龄;
        self
    }

    pub fn 设置分数(&mut self, 分数: Vec<u32>) -> &mut Self {
        self.分数 = 分数;
        self
    }

    pub fn 设置成绩(&mut self, 成绩: HashMap<String, u32>) -> &mut Self {
        self.成绩 = 成绩;
        self
    }

    pub fn 添加学科(&mut self, 新学科: Vec<String>) -> &mut Self {
        // let 已有: HashSet<&String> = self.学科.iter().collect();

        let 已有: HashSet<String> = self.学科.iter().cloned().collect();

        let 待添加: Vec<String> = 新学科.into_iter().filter(|s| !已有.contains(s)).collect();
        self.学科.extend(待添加);
        self
    }
    pub fn 查看学生全部(&self) -> &Self {
        self
    }
}
