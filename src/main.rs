use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
mod student;
use student::Student;

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    let xiao_hong = Student::new().name("小红".into());
    let xiao_ming = Student::new().name("小明".into()).age(19).add_subjects(
        vec!["数学".to_string(), "英语".to_string()]
            .into_iter()
            .collect(),
    );
    let xiao_wang = Student::new()
        .name("小王".into())
        .add_subjects(
            vec!["语文".to_string(), "数学".to_string()]
                .into_iter()
                .collect(),
        )
        .add_subjects(
            vec!["数学".to_string(), "英语".to_string()]
                .into_iter()
                .collect(),
        );

    let students = Arc::new(vec![xiao_hong, xiao_ming, xiao_wang]);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let route = Router::new()
        .route("/", get(get_students))
        .with_state(students);

    axum::serve(listener, route).await.unwrap();
}

async fn get_students(State(students): State<Arc<Vec<Student>>>) -> Json<Arc<Vec<Student>>> {
    Json(students)
}
