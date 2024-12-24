use std::fmt::Display;

use chrono::{DateTime, Duration, Local};
use rusqlite::{params, types::Type, Connection};

use crate::{
    cli::Weight,
    error::{Error, Result},
};

pub struct Repo {
    conn: Connection,
}

impl Repo {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
        let todo_dir = home_dir.join(".todo");
        std::fs::create_dir_all(&todo_dir)?;
        let db_path = todo_dir.join("todos.db");

        let conn = Connection::open(db_path)?;

        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        conn.execute_batch(
            "BEGIN;
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                weight TEXT NOT NULL,
                start_date TEXT,
                deadline TEXT,
                completed BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                completed_at TEXT,
                CHECK (weight IN ('low', 'medium', 'high'))
            );
            CREATE INDEX IF NOT EXISTS idx_todos_name ON todos(name);
            CREATE INDEX IF NOT EXISTS idx_todos_completed ON todos(completed);
            CREATE INDEX IF NOT EXISTS idx_todos_deadline ON todos(deadline);
            COMMIT;",
        )?;

        Ok(Self { conn })
    }

    pub fn add(
        &mut self,
        name: String,
        description: Option<String>,
        weight: Option<Weight>,
        days_to_start: Option<u32>,
        days_to_complete: Option<u32>,
    ) -> Result<()> {
        let start_date = days_to_start.map(|days| Local::now() + Duration::days(days as i64));

        let deadline = days_to_complete.map(|days| Local::now() + Duration::days(days as i64));

        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT INTO todos (
                name, description, weight, start_date, deadline, completed, created_at
            ) VALUES (?, ?, ?, ?, ?, 0, ?)",
            params![
                name,
                description,
                weight.unwrap_or(Weight::Medium).to_string(),
                start_date.map(|d| d.to_rfc3339()),
                deadline.map(|d| d.to_rfc3339()),
                Local::now().to_rfc3339(),
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        let tx = self.conn.transaction()?;

        let rows_affected = tx.execute("DELETE FROM todos WHERE name = ?", params![name])?;

        if rows_affected == 0 {
            return Err(Error::ItemNotFound(name.to_string()));
        }

        tx.commit()?;
        Ok(())
    }

    pub fn edit(
        &mut self,
        name: String,
        new_name: Option<String>,
        description: Option<String>,
        weight: Option<Weight>,
        days_to_start: Option<u32>,
        days_to_complete: Option<u32>,
    ) -> Result<()> {
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(new_name) = new_name {
            updates.push("name = ?");
            params.push(new_name);
        }
        if let Some(description) = description {
            updates.push("description = ?");
            params.push(description);
        }
        if let Some(weight) = weight {
            updates.push("weight = ?");
            params.push(weight.to_string());
        }
        if let Some(days) = days_to_start {
            updates.push("start_date = ?");
            let date = Local::now() + Duration::days(days as i64);
            params.push(date.to_rfc3339());
        }
        if let Some(days) = days_to_complete {
            updates.push("deadline = ?");
            let date = Local::now() + Duration::days(days as i64);
            params.push(date.to_rfc3339());
        }

        if updates.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;

        let query = format!("UPDATE todos SET {} WHERE name = ?", updates.join(", "));
        params.push(name.clone());

        let rows_affected = tx.execute(&query, rusqlite::params_from_iter(params))?;

        if rows_affected == 0 {
            return Err(Error::ItemNotFound(name));
        }

        tx.commit()?;
        Ok(())
    }

    pub fn complete(&mut self, name: &str) -> Result<()> {
        let tx = self.conn.transaction()?;

        let exists = tx
            .query_row(
                "SELECT completed FROM todos WHERE name = ?",
                params![name],
                |row| row.get::<_, bool>(0),
            )
            .map_or(false, |completed| !completed);

        if !exists {
            return Err(Error::ItemNotFound(name.to_string()));
        }

        tx.execute(
            "UPDATE todos SET completed = 1, completed_at = ? WHERE name = ?",
            params![Local::now().to_rfc3339(), name],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn list(
        &self,
        weight: Option<Weight>,
        completed: bool,
        sort_by_deadline: bool,
        sort_by_weight: bool,
    ) -> Result<Vec<Item>> {
        let mut query = String::from("SELECT * FROM todos WHERE 1=1");
        let mut params: Vec<String> = Vec::new();

        if let Some(weight) = weight {
            query.push_str(" AND weight = ?");
            params.push(weight.to_string());
        }

        if completed {
            query.push_str(" AND completed = 1");
        }

        query.push_str(" ORDER BY ");
        if sort_by_deadline {
            query.push_str("COALESCE(deadline, '9999-12-31T23:59:59Z')");
        } else if sort_by_weight {
            query.push_str(
                "CASE weight 
                WHEN 'high' THEN 1 
                WHEN 'medium' THEN 2 
                WHEN 'low' THEN 3 
                END",
            );
        } else {
            query.push_str("created_at DESC");
        }

        let mut stmt = self.conn.prepare(&query)?;
        let todo_iter = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                weight: match row.get::<_, String>(3)?.as_str() {
                    "low" => Weight::Low,
                    "medium" => Weight::Medium,
                    "high" => Weight::High,
                    w => {
                        return Err(rusqlite::Error::InvalidColumnType(
                            3,
                            format!("Invalid weight value: {}", w),
                            Type::Text,
                        ))
                    }
                },
                start_date: row
                    .get::<_, Option<String>>(4)?
                    .map(|d| DateTime::parse_from_rfc3339(&d).map_err(map_chrono_error(4)))
                    .transpose()?
                    .map(|d| d.with_timezone(&Local)),
                deadline: row
                    .get::<_, Option<String>>(5)?
                    .map(|d| DateTime::parse_from_rfc3339(&d).map_err(map_chrono_error(5)))
                    .transpose()?
                    .map(|d| d.with_timezone(&Local)),
                completed: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map_err(map_chrono_error(7))?
                    .with_timezone(&Local),
                completed_at: row
                    .get::<_, Option<String>>(8)?
                    .map(|d| DateTime::parse_from_rfc3339(&d).map_err(map_chrono_error(8)))
                    .transpose()?
                    .map(|d| d.with_timezone(&Local)),
            })
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }

        Ok(todos)
    }
}

fn map_chrono_error(column: usize) -> impl Fn(chrono::ParseError) -> rusqlite::Error {
    move |err: chrono::ParseError| {
        rusqlite::Error::InvalidColumnType(
            column,
            format!("Invalid date format: {}", err),
            Type::Text,
        )
    }
}

#[derive(Debug)]
pub struct Item {
    #[allow(dead_code)]
    id: i64,
    name: String,
    description: Option<String>,
    weight: Weight,
    start_date: Option<DateTime<Local>>,
    deadline: Option<DateTime<Local>>,
    completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

impl Item {
    pub fn format_date(date: Option<DateTime<Local>>) -> String {
        date.map(|d| d.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "Not set".to_string())
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = Vec::new();
        let status = if self.completed { "✓" } else { " " };

        output.push(format!("[{}] {} ({})", status, self.name, self.weight));
        if let Some(desc) = &self.description {
            output.push(format!("    Description: {}", desc));
        }
        output.push(format!("    Start: {}", Self::format_date(self.start_date)));
        output.push(format!(
            "    Deadline: {}",
            Self::format_date(self.deadline)
        ));
        output.push(format!(
            "    Created: {}",
            self.created_at.format("%Y-%m-%d %H:%M")
        ));

        if self.completed {
            if let Some(completed_at) = self.completed_at {
                output.push(format!(
                    "    Completed: {}",
                    completed_at.format("%Y-%m-%d %H:%M")
                ));
            }
        }

        write!(f, "{}", output.join("\n"))
    }
}
