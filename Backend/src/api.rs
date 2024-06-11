use crate::*;
use axum::{
    extract::{Path, Extension},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn fetch_all_questions(
    Extension(question_list): Extension<Arc<QuestionList>>
) -> impl IntoResponse {
    let questions = question_list.get_all_questions().await;
    (StatusCode::OK, Json(questions))
}

pub async fn create_question(
    Extension(question_list): Extension<Arc<QuestionList>>,
    Json(new_question): Json<Question>
) -> impl IntoResponse {
    question_list.add_question(new_question).await;
    (StatusCode::CREATED, "Question added successfully")
}

pub async fn fetch_question(
    Extension(question_list): Extension<Arc<QuestionList>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match question_list.find_question(&id).await {
        Some(question) => (StatusCode::OK, Json(question)).into_response(),
        None => (StatusCode::NOT_FOUND, "Question not found").into_response(),
    }
}

pub async fn update_question(
    Extension(question_list): Extension<Arc<QuestionList>>,
    Path(id): Path<String>,
    Json(updated_question): Json<Question>
) -> impl IntoResponse {
    match question_list.update_question(&id, updated_question).await {
        Ok(_) => (StatusCode::OK, "Question updated successfully").into_response(),
        Err(err) => (StatusCode::NOT_FOUND, err).into_response(),
    }
}

pub async fn remove_question(
    Extension(question_list): Extension<Arc<QuestionList>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    if question_list.remove_question(&id).await.is_some() {
        (StatusCode::OK, "Question deleted successfully").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Question not found").into_response()
    }
}
