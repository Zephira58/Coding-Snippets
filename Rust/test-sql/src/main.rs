use sqlite::State;

fn main() {
    // Connect to the db
    let connection = sqlite::open("./sqlite.db").unwrap();

    // Query string formatted in SQL
    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
";
    // Calls the query to be ran in the database
    connection.execute(query).unwrap();

    // Un-optimized data retrieval
    let query = "SELECT * FROM users WHERE age > 50";
    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .unwrap();

    // Optimized data retrieval
    let query = "SELECT * FROM users WHERE age > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, 50)).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }
}
