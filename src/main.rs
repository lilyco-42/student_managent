use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post, put},
};
use tower_http::cors::{Any, CorsLayer};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
mod student;
use student::{Student, StudentList};

#[derive(Clone)]
struct AppState {
    student_list: Arc<Mutex<StudentList>>,
}

// GET /students - 获取所有学生
async fn list_students(State(state): State<AppState>) -> Json<Vec<Student>> {
    let list = state.student_list.lock().unwrap();
    Json(list.list().clone())
}

// POST /students - 创建学生
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

// GET /student/{id} -查看学生
async fn get_student(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Student>, StatusCode> {
    let list = state.student_list.lock().unwrap();
    list.get_by_id(id)
        .map(|s| Json(s.clone())) // 如果 get_by_id 返回 &Student 则需要克隆
        .ok_or(StatusCode::NOT_FOUND)
}

// DELETE /students/:id - 删除学生
async fn delete_student(State(state): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let mut list = state.student_list.lock().unwrap();
    if list.delete_by_id(id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// PUT /students/:id - 更新学生
async fn update_student(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(updated): Json<Student>,
) -> StatusCode {
    if id != updated.id {
        return StatusCode::BAD_REQUEST;
    }
    let mut list = state.student_list.lock().unwrap();
    if list.update(updated) {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
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
        .route(
            "/students/{id}",
            get(get_student).delete(delete_student).put(update_student),
        )
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
async fn root_handler() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>学生管理系统 - 前端测试面板</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
    <style>
        .student-card {
            transition: transform 0.2s;
            margin-bottom: 1rem;
        }
        .student-card:hover {
            transform: translateY(-3px);
            box-shadow: 0 4px 8px rgba(0,0,0,0.1);
        }
    </style>
</head>
<body>
    <div class="container mt-5" id="root"></div>

    <script src="https://cdn.jsdelivr.net/npm/react@18.2.0/umd/react.development.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/react-dom@18.2.0/umd/react-dom.development.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/axios@1.6.2/dist/axios.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>

    <script>
        // 基础 API 地址（可根据实际情况调整）
        const API_BASE = '/students';

        // 学生管理组件
        const StudentManager = () => {
            const [students, setStudents] = React.useState([]);
            const [loading, setLoading] = React.useState(false);
            const [formData, setFormData] = React.useState({
                id: '',
                name: '',
                age: '',
                birthdate: '',
                institute: '',
                score: ''
            });
            const [editingId, setEditingId] = React.useState(null);

            // 获取所有学生
            const fetchStudents = async () => {
                setLoading(true);
                try {
                    const res = await axios.get(API_BASE);
                    setStudents(res.data);
                } catch (err) {
                    console.error('获取学生列表失败', err);
                    alert('获取学生列表失败，请检查后端是否运行');
                } finally {
                    setLoading(false);
                }
            };

            // 添加学生
            const addStudent = async (student) => {
                try {
                    await axios.post(API_BASE, student);
                    fetchStudents(); // 刷新列表
                    return true;
                } catch (err) {
                    if (err.response && err.response.status === 409) {
                        alert('学生ID已存在，请修改ID');
                    } else {
                        alert('添加失败');
                    }
                    return false;
                }
            };

            // 更新学生
            const updateStudent = async (id, student) => {
                try {
                    await axios.put(`${API_BASE}/${id}`, student);
                    fetchStudents();
                    return true;
                } catch (err) {
                    if (err.response && err.response.status === 404) {
                        alert('学生不存在');
                    } else {
                        alert('更新失败');
                    }
                    return false;
                }
            };

            // 删除学生
            const deleteStudent = async (id) => {
                if (!confirm('确定删除该学生吗？')) return;
                try {
                    await axios.delete(`${API_BASE}/${id}`);
                    fetchStudents();
                } catch (err) {
                    if (err.response && err.response.status === 404) {
                        alert('学生不存在');
                    } else {
                        alert('删除失败');
                    }
                }
            };

            // 处理表单输入
            const handleChange = (e) => {
                setFormData({ ...formData, [e.target.name]: e.target.value });
            };

            // 提交表单（新增或更新）
            const handleSubmit = async (e) => {
                e.preventDefault();
                const student = {
                    id: parseInt(formData.id),
                    name: formData.name,
                    age: parseInt(formData.age),
                    birthdate: formData.birthdate,
                    institute: formData.institute,
                    score: parseInt(formData.score)
                };
                if (editingId !== null) {
                    // 更新模式
                    const success = await updateStudent(editingId, student);
                    if (success) {
                        setEditingId(null);
                        setFormData({ id: '', name: '', age: '', birthdate: '', institute: '', score: '' });
                    }
                } else {
                    // 新增模式
                    const success = await addStudent(student);
                    if (success) {
                        setFormData({ id: '', name: '', age: '', birthdate: '', institute: '', score: '' });
                    }
                }
            };

            // 编辑学生（填充表单）
            const editStudent = (student) => {
                setEditingId(student.id);
                setFormData({
                    id: student.id,
                    name: student.name,
                    age: student.age,
                    birthdate: student.birthdate,
                    institute: student.institute,
                    score: student.score
                });
            };

            React.useEffect(() => {
                fetchStudents();
            }, []);

            return React.createElement('div', null,
                React.createElement('h1', { className: 'mb-4' }, '📚 学生管理系统'),
                React.createElement('div', { className: 'row' },
                    React.createElement('div', { className: 'col-md-5' },
                        React.createElement('div', { className: 'card shadow-sm' },
                            React.createElement('div', { className: 'card-body' },
                                React.createElement('h5', { className: 'card-title' }, editingId !== null ? '编辑学生' : '添加新学生'),
                                React.createElement('form', { onSubmit: handleSubmit },
                                    ['id', 'name', 'age', 'birthdate', 'institute', 'score'].map(field =>
                                        React.createElement('div', { className: 'mb-3', key: field },
                                            React.createElement('label', { className: 'form-label' }, field === 'birthdate' ? '生日 (YYYY-MM-DD)' : field),
                                            React.createElement('input', {
                                                type: field === 'age' || field === 'score' ? 'number' : field === 'birthdate' ? 'date' : 'text',
                                                className: 'form-control',
                                                name: field,
                                                value: formData[field],
                                                onChange: handleChange,
                                                required: true
                                            })
                                        )
                                    ),
                                    React.createElement('button', { type: 'submit', className: 'btn btn-primary me-2' },
                                        editingId !== null ? '更新学生' : '添加学生'
                                    ),
                                    editingId !== null && React.createElement('button', {
                                        type: 'button',
                                        className: 'btn btn-secondary',
                                        onClick: () => {
                                            setEditingId(null);
                                            setFormData({ id: '', name: '', age: '', birthdate: '', institute: '', score: '' });
                                        }
                                    }, '取消')
                                )
                            )
                        )
                    ),
                    React.createElement('div', { className: 'col-md-7' },
                        React.createElement('div', { className: 'card shadow-sm' },
                            React.createElement('div', { className: 'card-body' },
                                React.createElement('h5', { className: 'card-title' }, '学生列表'),
                                loading ? React.createElement('p', null, '加载中...') :
                                    students.length === 0 ? React.createElement('p', null, '暂无学生') :
                                        React.createElement('div', { className: 'row' },
                                            students.map(student =>
                                                React.createElement('div', { key: student.id, className: 'col-md-6' },
                                                    React.createElement('div', { className: 'card student-card' },
                                                        React.createElement('div', { className: 'card-body' },
                                                            React.createElement('h6', { className: 'card-title' }, `${student.name} (ID: ${student.id})`),
                                                            React.createElement('p', { className: 'card-text small' },
                                                                `年龄: ${student.age} | 生日: ${student.birthdate}<br>
                                                                学院: ${student.institute} | 分数: ${student.score}`
                                                            ),
                                                            React.createElement('button', {
                                                                className: 'btn btn-sm btn-warning me-2',
                                                                onClick: () => editStudent(student)
                                                            }, '编辑'),
                                                            React.createElement('button', {
                                                                className: 'btn btn-sm btn-danger',
                                                                onClick: () => deleteStudent(student.id)
                                                            }, '删除')
                                                        )
                                                    )
                                                )
                                            )
                                        )
                            )
                        )
                    )
                )
            );
        };

        // 渲染到根元素
        const root = ReactDOM.createRoot(document.getElementById('root'));
        root.render(React.createElement(StudentManager));
    </script>
</body>
</html>
    "#,
    )
}
