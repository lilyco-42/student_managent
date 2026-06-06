## 观察axum demo
- 我们可以发现，axum 声明get请求和路由 后挂到handler函数去处理我们的数据
```
route("/", get(|| async { "Hello, World!" }));
```
- 也就是把get(函数)的函数换成我们的增删改查就可以了
- 查阅tokio 文档可知有一个state用来异步同步我们各个接口的状态和数据


## 我们从最简单的数据设计开始

```

struct Student {
    id: u32,
    name: String,
    score: u32,
    insitute : String,
    birth : String,
    age : u32,


}


pub struct StudentList {
    pub students: Vec<Student>,
}
```
- 我们设计了一个学生结构体,和一个动态数组学生们,结构体学生列表是公开的
- 他有 学号，名字，得分，学院，出生日期，年龄

```

impl StudentList {
   pub fn add(&mut self, student: Student) -> &mut Self {
        self.students.push(student);
        self
    }
    pub fn list(&self) -> &[Student] {
        &self.students
    }
    pub fn new() -> Self {
        self::Default()
    }
}
```
- impl关联学生列表结构，获取列表自身self,push添加我们的学生，返回学生列表
- 给他们加上派生宏。#[derive(Debug, Clone, Serialize, Deserialize)]
- 转换成web返回的json 格式
```
#[derive(Debug, Clone, Default)]
pub struct StudentList {
    pub students: Arc<Mutex<Vec<Student>>>,
}
pub type SharedStudents = Arc<Mutex<Vec<Student>>>;
#[derive(Clone)]
pub struct AppState {
    pub students: SharedStudents,
}
```
- 多线程用arc，加锁Mutex
