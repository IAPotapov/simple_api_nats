use crate::model::{Command, Todo, Response, DB};

use chrono::prelude::*;
use uuid::Uuid;

pub fn health_checker_handler(cmd: &Command) -> Response
{
    let r = Response
    {
        id: cmd.id.clone(),
        error: 0,
        message: Some("success".to_string()),
        data: None,
    };
    return r;
}

pub fn todos_list_handler(cmd: &Command, db: &DB) -> Response
{
    let todos: Vec<Todo> = db.clone().into_iter().collect();

    let r = Response
    {
        id: cmd.id.clone(),
        error: 0,
        message: None,
        data: Some(todos),
    };
    return  r;
}

pub fn create_todo_handler(cmd: &Command, db: &mut DB) -> Response
{
    if cmd.data.is_none()
    {
        let e = Response
        {
            id: cmd.id.clone(),
            error: 1,
            message: Some("Todo argument is missing".to_string()),
            data: None,
        };
        return e;
    }
    let mut body = cmd.data.clone().unwrap();
    for t in db.iter()
    {
        if t.title == body.title
        {
            let e = Response
            {
                id: cmd.id.clone(),
                error: 1,
                message: Some(format!("Todo with title: '{}' already exists", t.title)),
                data: None,
            };
            return e;
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let mut data: Vec<Todo> = Vec::new();
    data.push(body.clone());
    db.push(body);

    let r = Response
    {
        id: cmd.id.clone(),
        error: 0,
        message: None,
        data: Some(data),
    };
    return r;
}

pub fn get_todo_handler(cmd: &Command, db: &mut DB) -> Response
{
    if cmd.data.is_none()
    {
        let e = Response
        {
            id: cmd.id.clone(),
            error: 1,
            message: Some("Todo argument is missing".to_string()),
            data: None,
        };
        return e;
    }
    let body = cmd.data.clone().unwrap();
    
    for t in db.iter()
    {
        if t.id == body.id
        {
            let mut data: Vec<Todo> = Vec::new();
            data.push(t.clone());
            let r = Response
            {
                id: cmd.id.clone(),
                error: 0,
                message: None,
                data: Some(data),
            };
            return r;
        }
    }
    let e = Response
    {
        id: cmd.id.clone(),
        error: 1,
        message: Some(format!("Todo with ID: {} not found", body.id.unwrap())),
        data: None,
    };
    return e;
}

pub fn edit_todo_handler(cmd: &Command, db: &mut DB) -> Response
{
    if cmd.data.is_none()
    {
        let e = Response
        {
            id: cmd.id.clone(),
            error: 1,
            message: Some("Todo argument is missing".to_string()),
            data: None,
        };
        return e;
    }
    let body = cmd.data.clone().unwrap();
    for t in db.iter_mut()
    {
        if t.id == body.id
        {
            let datetime = Utc::now();
            let payload = Todo
            {
                id: t.id.to_owned(),
                title: body.title.to_owned(),
                content: body.content.to_owned(),
                completed: body.completed.to_owned(),
                created_at: t.created_at,
                updated_at: Some(datetime),
            };
            *t = payload;
            let mut data: Vec<Todo> = Vec::new();
            data.push(t.clone());
            let r = Response
            {
                id: cmd.id.clone(),
                error: 0,
                message: None,
                data: Some(data),
            };
            return r;
        }
    }
    let e = Response
    {
        id: cmd.id.clone(),
        error: 1,
        message: Some(format!("Todo with ID: {} not found", body.id.unwrap())),
        data: None,
    };
    return e;
}

pub fn delete_todo_handler(cmd: &Command, db: &mut DB) -> Response
{
    if cmd.data.is_none()
    {
        let e = Response
        {
            id: cmd.id.clone(),
            error: 1,
            message: Some("Todo argument is missing".to_string()),
            data: None,
        };
        return e;
    }
    let body = cmd.data.clone().unwrap();

    for t in db.iter()
    {
        if t.id == body.id
        {
            db.retain(|t| t.id != body.id);
            let r = Response
            {
                id: cmd.id.clone(),
                error: 0,
                message: None,
                data: None,
            };
            return r;
        }
    }

    let e = Response
    {
        id: cmd.id.clone(),
        error: 1,
        message: Some(format!("Todo with ID: {} not found", body.id.unwrap())),
        data: None,
    };
    return e;
}