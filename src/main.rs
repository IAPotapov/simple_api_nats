mod model;
mod handler;

use async_nats::Client;
use futures::StreamExt;
use handler::*;
use model::{Command, DB};
use std::str::from_utf8;
use serde_json;

async fn process_command(cmd: &Command, client: &Client, db: &mut DB) -> Result<(), async_nats::Error>
{
    println!("Command: {}", cmd.command);
    if cmd.command == "health_checker"
    {
        let r = health_checker_handler(cmd);
        let j = serde_json::to_string(&r)?;
        client.publish("todos.response", j.into()).await?;
    }
    if cmd.command == "todos_list"
    {
        let r = todos_list_handler(cmd, db);
        let j = serde_json::to_string(&r)?;
        client.publish("todos.response", j.into()).await?;
    }
    if cmd.command == "create_todo"
    {
        let r = create_todo_handler(cmd, db);
        let j = serde_json::to_string(&r)?;
        client.publish("todos.response", j.into()).await?;
    }
    if cmd.command == "get_todo"
    {
        let r = get_todo_handler(cmd, db);
        let j = serde_json::to_string(&r)?;
        client.publish("todos.response", j.into()).await?;
    }
    if cmd.command == "edit_todo"
    {
        let r = edit_todo_handler(cmd, db);
        let j = serde_json::to_string(&r)?;
        client.publish("todos.response", j.into()).await?;
    }
    if cmd.command == "delete_todo"
    {
        let r = delete_todo_handler(cmd, db);
        let j = serde_json::to_string(&r)?;
        client.publish("todos.response", j.into()).await?;
    }

    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> 
{
    let mut db = model::new_db();

    let nats_url = "nats://localhost:4222".to_string();
    let client = async_nats::connect(nats_url).await?;
    let mut subscription = client.subscribe("todos.command").await?;

    while let Some(message) = subscription.next().await
    {
        match from_utf8(&message.payload)
        {
            Err(e) => println!("Problem parsing message: {e:?}"),
            Ok(s) =>
            {
                let cmd: Command = serde_json::from_str(s)?;
                process_command(&cmd, &client, &mut db).await?;
            }
        }
    }

    return Ok(());
}
