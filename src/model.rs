use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command
{
    pub id: String,
    pub command: String,
    pub data: Option<Todo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response
{
    pub id: String,
    pub error: i32,
    pub message: Option<String>,
    pub data: Option<Vec<Todo>>,
}

pub type DB = Vec<Todo>;

pub fn new_db() -> DB 
{
    return Vec::new();
}