use sqlx::{sqlite::{SqlitePool, SqliteConnectOptions}, FromRow};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
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
    #[serde(default)]
    pub return_to: Option<String>,
    #[serde(default)]
    pub execution_deadline: Option<String>,
    #[serde(default)]
    pub execution_period_type: Option<String>,
    #[serde(default)]
    pub extended_deadline: Option<String>,
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub department: Option<String>,
    #[serde(default)]
    pub controller: Option<String>,
    #[serde(default)]
    pub controller_user_id: Option<i64>,
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
        // Отключаемся от предыдущего подключения если есть
        if self.pool.is_some() {
            self.disconnect();
        }

        let path = Path::new(db_path);
        
        // Создаем директорию если нужно
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Создаем новый runtime
        let runtime = Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        // Проверяем, существует ли файл, и создаем пустой файл если нужно
        let db_file_path = Path::new(db_path);
        if !db_file_path.exists() {
            std::fs::File::create(db_path)
                .map_err(|e| format!("Failed to create database file: {}", e))?;
        }
        
        // Подключаемся к базе данных
        let pool = runtime.block_on(async {
            let normalized_path = db_path.replace('\\', "/");
            
            // Используем SqliteConnectOptions для настройки подключения
            let options = SqliteConnectOptions::from_str(&format!("sqlite:///{}", normalized_path))
                .map_err(|e| sqlx::Error::Configuration(format!("Invalid database path: {}", e).into()))?
                .create_if_missing(true)
                .busy_timeout(std::time::Duration::from_secs(5));
            
            SqlitePool::connect_with(options).await
        })
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

        // Инициализируем схему
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

        // Проверяем, существует ли таблица control_cards перед миграциями
        let table_exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='control_cards'"
        )
        .fetch_one(pool)
        .await?;

        // Выполняем миграции только если таблица существует
        if table_exists.0 > 0 {
            // Вспомогательная функция для безопасного добавления колонки
            // Просто пытаемся добавить и игнорируем ошибку, если колонка уже существует
            async fn try_add_column(pool: &SqlitePool, column_name: &str, column_type: &str) {
                let alter_query = format!(
                    "ALTER TABLE control_cards ADD COLUMN {} {}",
                    column_name, column_type
                );
                let _ = sqlx::query(&alter_query)
                    .execute(pool)
                    .await;
            }

            // Миграции: добавляем поля если их нет (игнорируем ошибки если уже существуют)
            try_add_column(pool, "user_id", "INTEGER").await;
            try_add_column(pool, "executor_user_id", "INTEGER").await;
            try_add_column(pool, "return_to", "TEXT").await;
            try_add_column(pool, "execution_deadline", "TEXT").await;
            try_add_column(pool, "execution_period_type", "TEXT").await;
            try_add_column(pool, "extended_deadline", "TEXT").await;
            try_add_column(pool, "resolution", "TEXT").await;
            try_add_column(pool, "department", "TEXT").await;
            try_add_column(pool, "controller", "TEXT").await;
            try_add_column(pool, "controller_user_id", "INTEGER").await;
        }

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
        let result = runtime
            .block_on(operation)
            .map_err(|e| format!("{}: {}", error_msg, e));
        result
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
        return_to: Option<&str>,
        execution_deadline: Option<&str>,
        execution_period_type: Option<&str>,
        extended_deadline: Option<&str>,
        resolution: Option<&str>,
        department: Option<&str>,
        controller: Option<&str>,
        controller_user_id: Option<i64>,
    ) -> Result<i64, String> {
        let pool = self.get_pool()?.clone();
        let executor = executor.to_string();
        let reporter = reporter.to_string();
        let summary = summary.to_string();
        let document_reference = document_reference.to_string();
        let return_to = return_to.map(|s| s.to_string());
        let execution_deadline = execution_deadline.map(|s| s.to_string());
        let execution_period_type = execution_period_type.map(|s| s.to_string());
        let extended_deadline = extended_deadline.map(|s| s.to_string());
        let resolution = resolution.map(|s| s.to_string());
        let department = department.map(|s| s.to_string());
        let controller = controller.map(|s| s.to_string());
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    INSERT INTO control_cards (card_number, year, executor, reporter, summary, document_reference, user_id, executor_user_id, return_to, execution_deadline, execution_period_type, extended_deadline, resolution, department, controller, controller_user_id)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
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
                .bind(&return_to)
                .bind(&execution_deadline)
                .bind(&execution_period_type)
                .bind(&extended_deadline)
                .bind(&resolution)
                .bind(&department)
                .bind(&controller)
                .bind(controller_user_id)
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
                    SELECT id, card_number, year, executor, reporter, summary, document_reference, executor_user_id, created_at, return_to, execution_deadline, execution_period_type, extended_deadline, resolution, department, controller, controller_user_id
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
                let result = sqlx::query_as::<_, ControlCard>(
                    r#"
                    SELECT id, card_number, year, executor, reporter, summary, document_reference, executor_user_id, created_at, return_to, execution_deadline, execution_period_type, extended_deadline, resolution, department, controller, controller_user_id
                    FROM control_cards
                    ORDER BY year DESC, card_number DESC
                    "#,
                )
                .fetch_all(&pool)
                .await;
                result
            },
            "Failed to get control cards",
        )
    }

    pub fn get_control_cards_by_executor_id(&self, executor_user_id: i64) -> Result<Vec<ControlCard>, String> {
        let pool = self.get_pool()?.clone();
        
        self.execute_async(
            async move {
                sqlx::query_as::<_, ControlCard>(
                    r#"
                    SELECT id, card_number, year, executor, reporter, summary, document_reference, executor_user_id, created_at, return_to, execution_deadline, execution_period_type, extended_deadline, resolution, department, controller, controller_user_id
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
        return_to: Option<&str>,
        execution_deadline: Option<&str>,
        execution_period_type: Option<&str>,
        extended_deadline: Option<&str>,
        resolution: Option<&str>,
        department: Option<&str>,
        controller: Option<&str>,
        controller_user_id: Option<i64>,
    ) -> Result<usize, String> {
        let pool = self.get_pool()?.clone();
        let executor = executor.to_string();
        let reporter = reporter.to_string();
        let summary = summary.to_string();
        let document_reference = document_reference.to_string();
        let return_to = return_to.map(|s| s.to_string());
        let execution_deadline = execution_deadline.map(|s| s.to_string());
        let execution_period_type = execution_period_type.map(|s| s.to_string());
        let extended_deadline = extended_deadline.map(|s| s.to_string());
        let resolution = resolution.map(|s| s.to_string());
        let department = department.map(|s| s.to_string());
        let controller = controller.map(|s| s.to_string());
        
        self.execute_async(
            async move {
                let result = sqlx::query(
                    r#"
                    UPDATE control_cards
                    SET card_number = ?1, year = ?2, executor = ?3, reporter = ?4, summary = ?5, document_reference = ?6, user_id = ?7, executor_user_id = ?8, return_to = ?9, execution_deadline = ?10, execution_period_type = ?11, extended_deadline = ?12, resolution = ?13, department = ?14, controller = ?15, controller_user_id = ?16
                    WHERE id = ?17
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
                .bind(&return_to)
                .bind(&execution_deadline)
                .bind(&execution_period_type)
                .bind(&extended_deadline)
                .bind(&resolution)
                .bind(&department)
                .bind(&controller)
                .bind(controller_user_id)
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
                let result = sqlx::query_as::<_, User>(
                    r#"
                    SELECT id, username, password_hash, role, created_at
                    FROM users
                    WHERE id = ?1
                    "#,
                )
                .bind(id)
                .fetch_optional(&pool)
                .await;
                result
            },
            "Failed to get user by id",
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
