// RSQLite
use rusqlite::{Connection, Result};
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32
}
// connect DB
pub fn connect_db() -> Result<Connection> {
    let conn = Connection::open("test.db")?;
// print connected if there is no error
    println!("Connected");
    Ok(conn)
}
//  Create table users
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}
// Insert Data
pub fn insert_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        ["mimi","36"],
    )?;
    Ok(())
}

fn main() {
    let conn = connect_db().unwrap();
    create_table(&conn).unwrap();
    insert_data(&conn).unwrap();
    let mut stmt = conn.prepare("SELECT id, name, age FROM users").unwrap();
//     print user
    let users = stmt.query_map([], |row| {
        Ok(User {
             id: row.get(0)?,
             name: row.get(1)?,
             age: row.get(2)?,
        })

    }).unwrap();
    for user in users {
         match user {
            Ok(user) => println!("User: {} - {} - {}", user.id, user.name, user.age),
            Err(e) => println!("Error: {}", e),
        }
    }

}