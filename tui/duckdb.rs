use duckdb::{params, Connection, Result};
let conn = Connection::open_in_memory()?;

// ========= Queries =========
// #[derive(Debug)]
// struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>,
// }

// conn.execute(
//     "INSERT INTO person (name, data) VALUES (?, ?)",
//     params![me.name, me.data],
// )?;

// let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
// let person_iter = stmt.query_map([], |row| {
//     Ok(Person {
//         id: row.get(0)?,
//         name: row.get(1)?,
//         data: row.get(2)?,
//     })
// })?;

// for person in person_iter {
//     println!("Found person {:?}", person.unwrap());
// }

// ========= Inserts =========
// fn insert_rows(conn: &Connection) -> Result<()> {
//     let mut app = conn.appender("foo")?;
//     app.append_rows([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])?;
//     Ok(())
// }
