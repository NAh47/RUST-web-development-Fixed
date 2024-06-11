use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Question {
    pub id: String,
    pub header: String,
    pub body: String,
    pub categories: Option<Vec<String>>,
}

impl Question {
    pub fn new(id: String, header: String, body: String, categories: Option<Vec<String>>) -> Self {
        Self { id, header, body, categories }
    }
}

#[derive(Clone)]
pub struct QuestionList {
    pool: PgPool,
}

impl QuestionList {
    pub async fn new(pool: PgPool) -> Self {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS questions (
                id VARCHAR PRIMARY KEY,
                header VARCHAR NOT NULL,
                body VARCHAR NOT NULL,
                categories VARCHAR[]
            );"
        )
        .execute(&pool)
        .await
        .unwrap();

        Self { pool }
    }

    pub async fn add_question(&self, question: Question) {
        sqlx::query(
            "INSERT INTO questions (id, header, body, categories) VALUES ($1, $2, $3, $4)"
        )
        .bind(&question.id)
        .bind(&question.header)
        .bind(&question.body)
        .bind(&question.categories)
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn get_all_questions(&self) -> Vec<Question> {
        sqlx::query_as::<_, Question>("SELECT * FROM questions")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn find_question(&self, id: &str) -> Option<Question> {
        sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    pub async fn update_question(&self, id: &str, new_question: Question) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE questions SET header = $1, body = $2, categories = $3 WHERE id = $4"
        )
        .bind(&new_question.header)
        .bind(&new_question.body)
        .bind(&new_question.categories)
        .bind(id)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Question not found".to_string()),
        }
    }

    pub async fn remove_question(&self, id: &str) -> Option<Question> {
        let question = self.find_question(id).await;
        if let Some(q) = question {
            sqlx::query("DELETE FROM questions WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .unwrap();
            Some(q)
        } else {
            None
        }
    }
}
