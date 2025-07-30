use rusqlite::Connection;
use core::fmt;
use std::{env, error::Error, fmt::Formatter, fs, io::Read};

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
        task TEXT NOT NULL,
        progress TEXT NOT NULL,
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
        })?.collect::<Result<Vec<_>, rusqlite::Error>>()?;
    
    Ok(tasks)
}







// let conn = Connection::open("instance/mydata.db")?;

//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS user (
//             id INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             age INTEGER
//         )",
//         [],
//     )?;

//     conn.execute(
//         "INSERT INTO user (name, age) VALUES (?1, ?2)",
//         (&"Alice", &30),
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, age FROM user")?;
//     let user_iter = stmt.query_map([], |row| {
//         Ok((
//             row.get::<_, i32>(0)?,
//             row.get::<_, String>(1)?,
//             row.get::<_, i32>(2)?,
//         ))
//     })?;

//     for user in user_iter {
//         println!("Found user {:?}", user?);
//     }