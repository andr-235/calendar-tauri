mod database;
mod models;
mod auth;

use database::{Database, ControlCard};
use models::User;
use auth::{hash_password, verify_password, generate_token, verify_token};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static DB: Lazy<Mutex<Database>> = Lazy::new(|| Mutex::new(Database::new()));

fn with_db<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce(&mut Database) -> Result<T, String>,
{
    let mut db = DB.lock().map_err(|e| e.to_string())?;
    let result = f(&mut db);
    result
}

fn with_db_immut<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce(&Database) -> Result<T, String>,
{
    let db = DB.lock().map_err(|e| e.to_string())?;
    f(&db)
}

#[tauri::command]
fn connect_database(db_path: String) -> Result<(), String> {
    with_db(|db| {
        db.connect(&db_path)?;
        Ok(())
    })
}

#[tauri::command]
fn disconnect_database() -> Result<(), String> {
    with_db(|db| {
        db.disconnect();
        Ok(())
    })
}

#[tauri::command]
fn get_database_path() -> Result<Option<String>, String> {
    with_db_immut(|db| Ok(db.get_path().cloned()))
}

#[tauri::command]
fn is_database_connected() -> Result<bool, String> {
    with_db_immut(|db| Ok(db.is_connected()))
}

#[tauri::command]
fn get_next_card_number(year: i32) -> Result<i32, String> {
    with_db_immut(|db| db.get_next_card_number(year))
}

#[tauri::command]
fn create_control_card(
    card_number: i32,
    year: i32,
    executor_user_id: i64,
    reporter: String,
    summary: String,
    document_reference: String,
    return_to: Option<String>,
    execution_deadline: Option<String>,
    execution_period_type: Option<String>,
    extended_deadline: Option<String>,
    resolution: Option<String>,
    department: Option<String>,
    controller: Option<String>,
    controller_user_id: Option<i64>,
    token: String,
) -> Result<i64, String> {
    let claims = verify_token(&token)?;
    let user_id = claims.sub;
    let user_role = claims.role;

    // Проверка прав: только admin или controller могут создавать карточки
    if user_role != "admin" && user_role != "controller" {
        return Err("Only admin or controller can create control cards".to_string());
    }

    with_db_immut(|db| {
        // Получаем username пользователя-исполнителя для отображения
        let executor_user = db.get_user_by_id(executor_user_id)?;
        
        // Проверяем, что пользователь существует и имеет роль user
        let executor_user = executor_user
            .ok_or_else(|| "Executor user not found".to_string())?;
        
        if executor_user.role != "user" {
            return Err("Executor must be a user with role 'user'".to_string());
        }

        let executor = executor_user.username;

        // Проверяем controller_user_id если указан
        if let Some(controller_id) = controller_user_id {
            let user = db.get_user_by_id(controller_id)?;
            if let Some(ref u) = user {
                if u.role != "controller" {
                    return Err("Controller must be a user with role 'controller'".to_string());
                }
            }
        }

        let result = db.create_control_card(
            card_number,
            year,
            &executor,
            &reporter,
            &summary,
            &document_reference,
            Some(user_id),
            Some(executor_user_id),
            return_to.as_deref(),
            execution_deadline.as_deref(),
            execution_period_type.as_deref(),
            extended_deadline.as_deref(),
            resolution.as_deref(),
            department.as_deref(),
            controller.as_deref(),
            controller_user_id,
        )?;
        Ok(result)
    })
}

#[tauri::command]
fn get_control_card(id: i64, token: String) -> Result<ControlCard, String> {
    let claims = verify_token(&token)?;
    let user_id = claims.sub;
    let user_role = claims.role;

    with_db_immut(|db| {
        let card = db.get_control_card(id)?;
        
        // Admin и controller видят все карточки
        if user_role == "admin" || user_role == "controller" {
            Ok(card)
        } else {
            // User видит только карточки, где он исполнитель
            match card.executor_user_id {
                Some(executor_id) if executor_id == user_id => Ok(card),
                Some(_) => Err("Access denied: you can only view cards where you are the executor".to_string()),
                None => Err("Card not found or has no executor assigned".to_string()),
            }
        }
    })
}

#[tauri::command]
fn get_all_control_cards(token: String) -> Result<Vec<ControlCard>, String> {
    let claims = verify_token(&token)?;
    let user_id = claims.sub;
    let user_role = claims.role;

    with_db_immut(|db| {
        // Admin и controller видят все карточки
        if user_role == "admin" || user_role == "controller" {
            db.get_all_control_cards()
        } else {
            // User видит только карточки, где он исполнитель
            db.get_control_cards_by_executor_id(user_id)
        }
    })
}

#[tauri::command]
fn update_control_card(
    id: i64,
    card_number: i32,
    year: i32,
    executor_user_id: i64,
    reporter: String,
    summary: String,
    document_reference: String,
    return_to: Option<String>,
    execution_deadline: Option<String>,
    execution_period_type: Option<String>,
    extended_deadline: Option<String>,
    resolution: Option<String>,
    department: Option<String>,
    controller: Option<String>,
    controller_user_id: Option<i64>,
    token: String,
) -> Result<usize, String> {
    let claims = verify_token(&token)?;
    let user_id = claims.sub;
    let user_role = claims.role;

    // Только admin или controller могут обновлять карточки
    if user_role != "admin" && user_role != "controller" {
        return Err("Only admin or controller can update control cards".to_string());
    }

    with_db_immut(|db| {
        // Получаем username пользователя-исполнителя для отображения
        let executor_user = db.get_user_by_id(executor_user_id)?;
        
        // Проверяем, что пользователь существует и имеет роль user
        let executor_user = executor_user
            .ok_or_else(|| "Executor user not found".to_string())?;
        
        if executor_user.role != "user" {
            return Err("Executor must be a user with role 'user'".to_string());
        }

        let executor = executor_user.username;

        // Проверяем controller_user_id если указан
        if let Some(controller_id) = controller_user_id {
            let user = db.get_user_by_id(controller_id)?;
            if let Some(ref u) = user {
                if u.role != "controller" {
                    return Err("Controller must be a user with role 'controller'".to_string());
                }
            }
        }

        db.update_control_card(
            id,
            card_number,
            year,
            &executor,
            &reporter,
            &summary,
            &document_reference,
            Some(user_id),
            Some(executor_user_id),
            return_to.as_deref(),
            execution_deadline.as_deref(),
            execution_period_type.as_deref(),
            extended_deadline.as_deref(),
            resolution.as_deref(),
            department.as_deref(),
            controller.as_deref(),
            controller_user_id,
        )
    })
}

#[tauri::command]
fn delete_control_card(id: i64, token: String) -> Result<usize, String> {
    let claims = verify_token(&token)?;
    let user_role = claims.role;

    // Только admin или controller могут удалять карточки
    if user_role != "admin" && user_role != "controller" {
        return Err("Only admin or controller can delete control cards".to_string());
    }

    with_db_immut(|db| {
        db.delete_control_card(id)
    })
}

#[tauri::command]
fn ensure_database_connected() -> Result<(), String> {
    // Вычисляем правильный путь БД
    let app_data_dir = std::env::var("APPDATA")
        .or_else(|_| std::env::var("LOCALAPPDATA"))
        .map_err(|_| "Failed to get app data directory".to_string())?;
    let db_dir = std::path::Path::new(&app_data_dir).join("calendar-tauri");
    std::fs::create_dir_all(&db_dir)
        .map_err(|e| format!("Failed to create database directory: {}", e))?;
    
    let default_db_path = db_dir.join("calendar.db");
    let correct_db_path_str = default_db_path.to_str()
        .ok_or_else(|| "Invalid database path".to_string())?;
    
    // Проверяем текущий путь БД
    let current_path = with_db_immut(|db| Ok(db.get_path().cloned()))?;
    
    // Если БД подключена к правильному пути, ничего не делаем
    if let Some(ref path) = current_path {
        // Нормализуем пути для сравнения
        let normalized_current = path.replace('\\', "/");
        let normalized_correct = correct_db_path_str.replace('\\', "/");
        
        if normalized_current == normalized_correct {
            return Ok(());
        } else {
            // БД подключена к неправильному пути, отключаем и переподключаем
            with_db(|db| {
                db.disconnect();
                Ok(())
            })?;
        }
    }
    
    // Подключаемся к правильному пути
    with_db(|db| {
        db.connect(correct_db_path_str)?;
        Ok(())
    })
}

#[tauri::command]
fn init_admin(username: String, password: String) -> Result<i64, String> {
    if password.len() < 6 {
        return Err("Password must be at least 6 characters long".to_string());
    }

    let password_hash = hash_password(&password)?;

    with_db(|db| {
        let existing_user = db.get_user_by_username(&username)?;
        if existing_user.is_some() {
            return Err("User already exists".to_string());
        }

        db.create_user(&username, &password_hash, "admin")
    })
}

#[tauri::command]
fn register_user(
    username: String,
    password: String,
    role: String,
    token: String,
) -> Result<i64, String> {
    let claims = verify_token(&token)?;
    if claims.role != "admin" {
        return Err("Only admin can register users".to_string());
    }

    if !matches!(role.as_str(), "admin" | "user" | "controller") {
        return Err("Invalid role".to_string());
    }

    if password.len() < 6 {
        return Err("Password must be at least 6 characters long".to_string());
    }

    let password_hash = hash_password(&password)?;

    with_db(|db| {
        db.create_user(&username, &password_hash, &role)
    })
}

#[tauri::command]
fn login(username: String, password: String) -> Result<String, String> {
    let user = with_db_immut(|db| {
        db.get_user_by_username(&username)
    })?;

    let user = user.ok_or_else(|| "Invalid username or password".to_string())?;

    let is_valid = verify_password(&password, &user.password_hash)?;
    if !is_valid {
        return Err("Invalid username or password".to_string());
    }

    generate_token(user.id, &user.role)
}

#[tauri::command]
fn get_current_user(token: String) -> Result<User, String> {
    let claims = verify_token(&token)?;
    
    with_db_immut(|db| {
        db.get_user_by_id(claims.sub)
            .and_then(|opt| opt.ok_or_else(|| "User not found".to_string()))
    })
}

#[tauri::command]
fn get_windows_username() -> Result<String, String> {
    Ok(whoami::username())
}

#[tauri::command]
fn has_any_users() -> Result<bool, String> {
    let is_connected = with_db_immut(|db| Ok(db.is_connected()))?;
    
    if !is_connected {
        return Ok(false);
    }
    
    with_db_immut(|db| {
        db.has_any_users()
    })
}

#[tauri::command]
fn get_all_users(token: String) -> Result<Vec<User>, String> {
    let claims = verify_token(&token)?;
    if claims.role != "admin" {
        return Err("Only admin can view all users".to_string());
    }

    with_db_immut(|db| {
        db.get_all_users()
    })
}

#[tauri::command]
fn get_users_for_executor_selection(token: String) -> Result<Vec<User>, String> {
    let claims = verify_token(&token)?;
    // Только admin или controller могут выбирать исполнителей
    if claims.role != "admin" && claims.role != "controller" {
        return Err("Only admin or controller can select executors".to_string());
    }

    with_db_immut(|db| {
        // Получаем всех пользователей с ролью 'user'
        let all_users = db.get_all_users()?;
        Ok(all_users.into_iter().filter(|u| u.role == "user").collect())
    })
}

#[tauri::command]
fn get_users_for_controller_selection(token: String) -> Result<Vec<User>, String> {
    let claims = verify_token(&token)?;
    // Только admin или controller могут выбирать контроллеров
    if claims.role != "admin" && claims.role != "controller" {
        return Err("Only admin or controller can select controllers".to_string());
    }

    with_db_immut(|db| {
        // Получаем всех пользователей с ролью 'controller'
        let all_users = db.get_all_users()?;
        Ok(all_users.into_iter().filter(|u| u.role == "controller").collect())
    })
}

#[tauri::command]
fn update_user(
    id: i64,
    username: String,
    role: String,
    token: String,
) -> Result<usize, String> {
    let claims = verify_token(&token)?;
    if claims.role != "admin" {
        return Err("Only admin can update users".to_string());
    }

    if !matches!(role.as_str(), "admin" | "user" | "controller") {
        return Err("Invalid role".to_string());
    }

    with_db_immut(|db| {
        let existing_user = db.get_user_by_id(id)?;
        if existing_user.is_none() {
            return Err("User not found".to_string());
        }

        let user_by_username = db.get_user_by_username(&username)?;
        if let Some(user) = user_by_username {
            if user.id != id {
                return Err("Username already exists".to_string());
            }
        }

        db.update_user(id, &username, &role)
    })
}

#[tauri::command]
fn delete_user(id: i64, token: String) -> Result<usize, String> {
    let claims = verify_token(&token)?;
    if claims.role != "admin" {
        return Err("Only admin can delete users".to_string());
    }

    with_db_immut(|db| {
        let existing_user = db.get_user_by_id(id)?;
        if existing_user.is_none() {
            return Err("User not found".to_string());
        }

        db.delete_user(id)
    })
}

#[tauri::command]
fn change_user_password(
    id: i64,
    new_password: String,
    token: String,
) -> Result<usize, String> {
    let claims = verify_token(&token)?;
    if claims.role != "admin" {
        return Err("Only admin can change user passwords".to_string());
    }

    if new_password.len() < 6 {
        return Err("Password must be at least 6 characters long".to_string());
    }

    let password_hash = hash_password(&new_password)?;

    with_db_immut(|db| {
        let existing_user = db.get_user_by_id(id)?;
        if existing_user.is_none() {
            return Err("User not found".to_string());
        }

        db.update_user_password(id, &password_hash)
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            connect_database,
            disconnect_database,
            get_database_path,
            is_database_connected,
            get_next_card_number,
            create_control_card,
            get_control_card,
            get_all_control_cards,
            update_control_card,
            delete_control_card,
            ensure_database_connected,
            init_admin,
            register_user,
            login,
            get_current_user,
            get_windows_username,
            has_any_users,
            get_all_users,
            get_users_for_executor_selection,
            get_users_for_controller_selection,
            update_user,
            delete_user,
            change_user_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
