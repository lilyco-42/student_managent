use std::collections::{HashMap, HashSet};

use axum::routing::get;
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

    let 端口 = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let 路由 = axum::Router::new().route("/", get(get学生列表()));
}

async fn get学生列表(学生列表: Vec<学生>) {
    for 学生 in 学生列表 {
        println!("{:#?}", 学生);
    }
}

#[derive(Debug, Default)]
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
