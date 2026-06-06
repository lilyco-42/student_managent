use axum::response::Html;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
mod student;
use student::{Student, StudentList};

#[derive(Clone)] // ✅ 添加 Clone
struct AppState {
    student_list: Arc<Mutex<StudentList>>,
}

async fn list_students(State(state): State<AppState>) -> Json<Vec<Student>> {
    let list = state.student_list.lock().unwrap();
    Json(list.list().clone())
}

async fn create_student(
    State(state): State<AppState>,
    Json(new_student): Json<Student>,
) -> StatusCode {
    let mut list = state.student_list.lock().unwrap();
    if list.list().iter().any(|s| s.id == new_student.id) {
        return StatusCode::CONFLICT;
    }
    list.add(new_student);
    StatusCode::CREATED
}

#[tokio::main]
async fn main() {
    let mut initial_list = StudentList::new();
    initial_list.add(Student {
        id: 1,
        name: "Alice".to_string(),
        age: 20,
        birthdate: "1999-01-01".to_string(),
        institute: "Institute A".to_string(),
        score: 85,
    });
    initial_list.add(Student {
        id: 2,
        name: "Bob".to_string(),
        age: 21,
        birthdate: "1998-01-01".to_string(),
        institute: "Institute B".to_string(),
        score: 90,
    });

    let state = AppState {
        student_list: Arc::new(Mutex::new(initial_list)),
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/students", get(list_students).post(create_student))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
async fn root_handler() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>Student API</title></head>
        <body>
            <h1>欢迎使用学生管理系统</h1>
            <p>请访问 <a href="/students">/students</a> 接口查看学生列表</p>
        </body>
        </html>
    "#,
    )
}
