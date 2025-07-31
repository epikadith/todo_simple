use rusqlite::Connection;
use core::fmt;
use std::{env, error::Error, fmt::Formatter, fs};

#[derive(Debug)]
pub enum MyError {
    BadInput
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::BadInput => write!(f, "bad input bro :("),
            _ => write!(f, "how did we get here")
        }
    }
}
impl Error for MyError {}


#[derive(Debug)]
pub struct List {
    pub id : u32,
    pub name : String
}

#[derive(Debug)]
pub struct Task {
    pub id : u32,
    pub name : String,
    pub prog : String,
    pub status : String
}

pub fn setup() -> Result<Connection, Box<dyn Error>>  {
    let pwd = env::current_dir()?;
    let new_dir = pwd.join("instance");
    fs::create_dir_all(&new_dir)?;
    let conn = Connection::open("instance/lists.db")?;
    conn.execute("
    CREATE TABLE IF NOT EXISTS lists (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )   
    ",[],)?;
    conn.execute("
    CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY,
        list_id INTEGER REFERENCES lists(id),
        name TEXT NOT NULL,
        prog TEXT NOT NULL,
        status TEXT NOT NULL
    )   
    "
    , [],)?;
    Ok(conn)
}

pub fn return_lists(conn: &Connection) -> Result<Vec<List>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, name FROM lists")?;
    let lists = stmt.query_map([],
         |row| {
            Ok(List{
                id : row.get(0)?,
                name : row.get(1)?
            })
        })?.collect::<Result<Vec<_>, rusqlite::Error>>()?;
    
    Ok(lists)
}

pub fn new_list(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("What do you want to name your new list?");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    if buf.len() == 0 {
        return Err(Box::new(MyError::BadInput));
    }
    conn.execute("INSERT INTO lists (name) VALUES (?1)", [&buf])?;
    println!("succesfully added list {}", &buf);
    
    Ok(())
}

pub fn return_tasks(conn: &Connection, list_id : u32) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT id, name, prog, status FROM tasks WHERE list_id = ?1")?;
    let tasks = stmt.query_map([list_id],
         |row| {
            Ok(Task{
                id : row.get(0)?,
                name : row.get(1)?,
                prog : row.get(2)?,
                status : row.get(3)?
            })
        })?.collect::<Result<Vec<Task>, rusqlite::Error>>()?;
    
    Ok(tasks)
}

pub fn new_task(conn: &Connection, n: u32) -> Result<(), Box<dyn Error>> {
    println!("What is your new task?");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    if buf.len() == 0 {
        return Err(Box::new(MyError::BadInput));
    }
    conn.execute("INSERT INTO tasks (list_id, name, prog, status) VALUES (?1, ?2, ?3, ?4)", (&n, &buf, &"NA", &"INCOMPLETE"))?;
    println!("succesfully added task {}", &buf);
    
    Ok(())
}

pub fn disp_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    println!("{:<4}{:<20}{:<20}{:<10}", "ID", "TASK", "PROGRESS", "STATUS");
    for task in tasks {
        println!("{:<4}{:<20}{:<20}{:<10}", task.id, task.name.trim(), task.prog.trim(), task.status);
    }
    Ok(())
}

pub fn update_task(conn: &Connection, task: &Task) -> Result<(), Box<dyn Error>> {
    println!("Current Progress: {}", task.prog);
    println!("What is the progress you've made?");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    if buf.len() == 0 {
        return Err(Box::new(MyError::BadInput));
    }
    conn.execute("UPDATE tasks SET prog = ?1 WHERE id = ?2", (&buf, &task.id))?;
    println!("Updated progress to {}", buf);
    Ok(())
}

pub fn delete_task(conn: &Connection, task: &Task) -> Result<(), Box<dyn Error>> {
    println!("Are you sure you want to delete this task: {}", task.name);
    println!("Click 0 for yes 1 for no");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    match buf.trim().parse::<u8>().unwrap() {
        0 => {
            conn.execute("DELETE FROM tasks WHERE id = ?1", [&task.id])?;
            println!("Deleted task successfuly");

        }
        _ => {
            return Ok(());
        }
    }
    Ok(())
}


pub fn mark_task(conn: &Connection, task: &Task) -> Result<(), Box<dyn Error>> {
    println!("Completed task: {}", task.name);
    println!("Click 0 to mark as completed, 1 to go back");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    match buf.trim().parse::<u8>().unwrap() {
        0 => {
            conn.execute("UPDATE tasks SET status = ?1 WHERE id = ?2", (&"COMPLETE", &task.id))?;
            println!("Marked as complete");
        }
        _ => {
            return Ok(());
        }
    }
    Ok(())
}
