use sqlx::{sqlite::SqlitePool, FromRow};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::runtime::Runtime;
use crate::models::User;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ControlCard {
    pub id: i64,
    pub card_number: i32,
    pub year: i32,
    pub executor: String,
    pub reporter: String,
    pub summary: String,
    pub document_reference: String,
    pub executor_user_id: Option<i64>,
    pub created_at: String,
}

pub struct Database {
    pool: Option<SqlitePool>,
    path: Option<String>,
    runtime: Option<Runtime>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            pool: None,
            path: None,
            runtime: None,
        }
    }

    pub fn connect(&mut self, db_path: &str) -> Result<(), String> {
        let path = Path::new(db_path);
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let runtime = Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        // Проверяем, существует ли файл, и создаем пустой файл если нужно
        let db_file_path = Path::new(db_path);
        if !db_file_path.exists() {
            std::fs::File::create(db_path)
                .map_err(|e| format!("Failed to create database file: {}", e))?;
        }
        
        let pool = runtime.block_on(async {
            let normalized_path = db_path.replace('\\', "/");
            let connection_string = format!("sqlite:///{}", normalized_path);
            
            SqlitePool::connect(&connection_string).await
        })
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

        runtime.block_on(async {
            self.init_schema(&pool).await
        })
        .map_err(|e| format!("Failed to initialize schema: {}", e))?;

        self.pool = Some(pool);
        self.path = Some(db_path.to_string());
        self.runtime = Some(runtime);
        
        Ok(())
    }

    pub fn disconnect(&mut self) {
        if let Some(runtime) = &self.runtime {
            if let Some(pool) = &self.pool {
                runtime.block_on(async {
                    pool.close().await;
                });
            }
        }
        self.pool = None;
        self.path = None;
        self.runtime = None;
    }

    pub fn is_connected(&self) -> bool {
        self.pool.is_some()
    }

    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    async fn init_schema(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS events")
            .execute(pool)
            .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL CHECK(role IN ('admin', 'user', 'controller')),
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS control_cards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                card_number INTEGER NOT NULL,
                year INTEGER NOT NULL,
                executor TEXT NOT NULL,
                reporter TEXT NOT NULL,
                summary TEXT NOT NULL,
                document_reference TEXT NOT NULL,
                user_id INTEGER,
                executor_user_id INTEGER,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                UNIQUE(year, card_number),
                FOREIGN KEY (user_id) REFERENCES users(id),
                FOREIGN KEY (executor_user_id) REFERENCES users(id)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Миграция: добавляем поле user_id если его нет
        sqlx::query(
            r#"
            ALTER TABLE control_cards ADD COLUMN user_id INTEGER
            "#,
        )
        .execute(pool)
        .await
        .ok(); // Игнорируем ошибку если колонка уже существует

        // Миграция: добавляем поле executor_user_id если его нет
        sqlx::query(
            r#"
            ALTER TABLE control_cards ADD COLUMN executor_user_id INTEGER
            "#,
        )
        .execute(pool)
        .await
        .ok(); // Игнорируем ошибку если колонка уже существует
        Ok(())
    }

    fn get_pool(&self) -> Result<&SqlitePool, String> {
        self.pool.as_ref().ok_or_else(|| "Database not connected".to_string())
    }

    fn get_runtime(&self) -> Result<&Runtime, String> {
        self.runtime.as_ref().ok_or_else(|| "Runtime not available".to_string())
    }

    fn execute_async<F, T>(&self, operation: F, error_msg: &str) -> Result<T, String>
    where
        F: std::future::Future<Output = Result<T, sqlx::Error>>,
    {
        let runtime = self.get_runtime()?;
        runtime
            .block_on(operation)
            .map_err(|e| format!("{}: {}", error_msg, e))
    }

    pub fn get_next_card_number(&self, year: i32) -> Result<i32, String> {
        let pool = self.get_pool()?.clone();
        let year = year;
        
        self.execute_async(
            async move {
                let result: Option<(i64,)> = sqlx::query_as(
                    r#"
                    SELECT MAX(card_number) as max_number
                    FROM control_cards
                    WHERE year = ?1
                    "#,
                )
                .bind(year)
                .fetch_optional(&pool)
                .await?;

                let next_number = match result {
                    Some((max,)) if max > 0 => (max as i32) + 1,
                    _ => 1,
                };
                Ok(next_number)
            },
            "Failed to get next card number",
        )
    }

    pub fn create_control_card(
        &self,
        card_number: i32,
        year: i32,
        executor: &str,
        reporter: &str,
        summary: &str,
        document_reference: &str,
        user_id: Option<i64>,
        executor_user_id: Option<i64>,
    ) -> Result<i64, String> {
        let pool = self.get_pool()?.clone();
        let executor = executor.to_string();
        let reporter = reporter.to_string();
        let summary = summary.to_string();
        let document_reference = document_reference.to_string();
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    INSERT INTO control_cards (card_number, year, executor, reporter, summary, document_reference, user_id, executor_user_id)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                    "#,
                )
                .bind(card_number)
                .bind(year)
                .bind(&executor)
                .bind(&reporter)
                .bind(&summary)
                .bind(&document_reference)
                .bind(user_id)
                .bind(executor_user_id)
                .execute(&pool)
                .await?;

                Ok(result.last_insert_rowid())
            },
            "Failed to create control card",
        )
    }

    pub fn get_control_card(&self, id: i64) -> Result<ControlCard, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, ControlCard>(
                    r#"
                    SELECT id, card_number, year, executor, reporter, summary, document_reference, executor_user_id, created_at
                    FROM control_cards
                    WHERE id = ?1
                    "#,
                )
                .bind(id)
                .fetch_one(&pool)
                .await
            },
            "Failed to get control card",
        )
    }

    pub fn get_all_control_cards(&self) -> Result<Vec<ControlCard>, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, ControlCard>(
                    r#"
                    SELECT id, card_number, year, executor, reporter, summary, document_reference, executor_user_id, created_at
                    FROM control_cards
                    ORDER BY year DESC, card_number DESC
                    "#,
                )
                .fetch_all(&pool)
                .await
            },
            "Failed to get control cards",
        )
    }

    pub fn get_control_cards_by_user_id(&self, user_id: i64) -> Result<Vec<ControlCard>, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, ControlCard>(
                    r#"
                    SELECT id, card_number, year, executor, reporter, summary, document_reference, executor_user_id, created_at
                    FROM control_cards
                    WHERE user_id = ?1
                    ORDER BY year DESC, card_number DESC
                    "#,
                )
                .bind(user_id)
                .fetch_all(&pool)
                .await
            },
            "Failed to get control cards by user_id",
        )
    }

    pub fn get_control_cards_by_executor_id(&self, executor_user_id: i64) -> Result<Vec<ControlCard>, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, ControlCard>(
                    r#"
                    SELECT id, card_number, year, executor, reporter, summary, document_reference, executor_user_id, created_at
                    FROM control_cards
                    WHERE executor_user_id = ?1
                    ORDER BY year DESC, card_number DESC
                    "#,
                )
                .bind(executor_user_id)
                .fetch_all(&pool)
                .await
            },
            "Failed to get control cards by executor_user_id",
        )
    }


    pub fn update_control_card(
        &self,
        id: i64,
        card_number: i32,
        year: i32,
        executor: &str,
        reporter: &str,
        summary: &str,
        document_reference: &str,
        user_id: Option<i64>,
        executor_user_id: Option<i64>,
    ) -> Result<usize, String> {
        let pool = self.get_pool()?.clone();
        let executor = executor.to_string();
        let reporter = reporter.to_string();
        let summary = summary.to_string();
        let document_reference = document_reference.to_string();
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    UPDATE control_cards
                    SET card_number = ?1, year = ?2, executor = ?3, reporter = ?4, summary = ?5, document_reference = ?6, user_id = ?7, executor_user_id = ?8
                    WHERE id = ?9
                    "#,
                )
                .bind(card_number)
                .bind(year)
                .bind(&executor)
                .bind(&reporter)
                .bind(&summary)
                .bind(&document_reference)
                .bind(user_id)
                .bind(executor_user_id)
                .bind(id)
                .execute(&pool)
                .await?;

                Ok(result.rows_affected() as usize)
            },
            "Failed to update control card",
        )
    }

    pub fn delete_control_card(&self, id: i64) -> Result<usize, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    DELETE FROM control_cards
                    WHERE id = ?1
                    "#,
                )
                .bind(id)
                .execute(&pool)
                .await?;

                Ok(result.rows_affected() as usize)
            },
            "Failed to delete control card",
        )
    }

    pub fn create_user(
        &self,
        username: &str,
        password_hash: &str,
        role: &str,
    ) -> Result<i64, String> {
        let pool = self.get_pool()?.clone();
        let username = username.to_string();
        let password_hash = password_hash.to_string();
        let role = role.to_string();
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    INSERT INTO users (username, password_hash, role)
                    VALUES (?1, ?2, ?3)
                    "#,
                )
                .bind(&username)
                .bind(&password_hash)
                .bind(&role)
                .execute(&pool)
                .await?;

                Ok(result.last_insert_rowid())
            },
            "Failed to create user",
        )
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let pool = self.get_pool()?.clone();
        let username = username.to_string();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, User>(
                    r#"
                    SELECT id, username, password_hash, role, created_at
                    FROM users
                    WHERE username = ?1
                    "#,
                )
                .bind(&username)
                .fetch_optional(&pool)
                .await
            },
            "Failed to get user by username",
        )
    }

    pub fn get_user_by_id(&self, id: i64) -> Result<Option<User>, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, User>(
                    r#"
                    SELECT id, username, password_hash, role, created_at
                    FROM users
                    WHERE id = ?1
                    "#,
                )
                .bind(id)
                .fetch_optional(&pool)
                .await
            },
            "Failed to get user by id",
        )
    }

    pub fn get_control_card_user_id(&self, id: i64) -> Result<Option<i64>, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                let result: Option<(Option<i64>,)> = sqlx::query_as(
                    r#"
                    SELECT user_id
                    FROM control_cards
                    WHERE id = ?1
                    "#,
                )
                .bind(id)
                .fetch_optional(&pool)
                .await?;

                Ok(result.and_then(|(user_id,)| user_id))
            },
            "Failed to get control card user_id",
        )
    }

    pub fn has_any_users(&self) -> Result<bool, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                let result: (i64,) = sqlx::query_as(
                    r#"
                    SELECT COUNT(*) as count
                    FROM users
                    "#,
                )
                .fetch_one(&pool)
                .await?;

                Ok(result.0 > 0)
            },
            "Failed to check if users exist",
        )
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, User>(
                    r#"
                    SELECT id, username, password_hash, role, created_at
                    FROM users
                    ORDER BY created_at DESC
                    "#,
                )
                .fetch_all(&pool)
                .await
            },
            "Failed to get all users",
        )
    }

    pub fn update_user(
        &self,
        id: i64,
        username: &str,
        role: &str,
    ) -> Result<usize, String> {
        let pool = self.get_pool()?.clone();
        let username = username.to_string();
        let role = role.to_string();
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    UPDATE users
                    SET username = ?1, role = ?2
                    WHERE id = ?3
                    "#,
                )
                .bind(&username)
                .bind(&role)
                .bind(id)
                .execute(&pool)
                .await?;

                Ok(result.rows_affected() as usize)
            },
            "Failed to update user",
        )
    }

    pub fn update_user_password(
        &self,
        id: i64,
        password_hash: &str,
    ) -> Result<usize, String> {
        let pool = self.get_pool()?.clone();
        let password_hash = password_hash.to_string();
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    UPDATE users
                    SET password_hash = ?1
                    WHERE id = ?2
                    "#,
                )
                .bind(&password_hash)
                .bind(id)
                .execute(&pool)
                .await?;

                Ok(result.rows_affected() as usize)
            },
            "Failed to update user password",
        )
    }

    pub fn delete_user(&self, id: i64) -> Result<usize, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    DELETE FROM users
                    WHERE id = ?1
                    "#,
                )
                .bind(id)
                .execute(&pool)
                .await?;

                Ok(result.rows_affected() as usize)
            },
            "Failed to delete user",
        )
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
