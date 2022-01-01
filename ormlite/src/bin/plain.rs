use ormlite::model::*;
use ormlite::Model;
use sqlx::Connection;
use std::str::FromStr;

#[derive(Model, sqlx::FromRow, Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

#[tokio::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = sqlx::SqliteConnection::connect_with(
        &sqlx::sqlite::SqliteConnectOptions::from_str("sqlite://:memory:").unwrap(),
    )
    .await?;
    env_logger::init();

    sqlx::query(ormlite::handwritten::CREATE_TABLE_SQL)
        .execute(&mut conn)
        .await?;

    // You can insert the model directly.
    let mut john = Person {
        id: 1,
        name: "John".to_string(),
        age: 99,
    }
    .insert(&mut conn)
    .await?;
    println!("{:?}", john);

    let people = Person::select()
        .filter("age > ?")
        .bind(50u32)
        .fetch_all(&mut conn)
        .await?;
    println!("select query builder {:?}", people);

    // TODO i want this to fail.
    let r = sqlx::query_as::<_, Person>("select * from person where age > ?")
        .fetch_all(&mut conn)
        .await?;
    println!("sqlx {:?}", r);

    // After modifying the object, you can update all fields directly.
    john.age = john.age + 1;
    john = john.update_all_fields(&mut conn).await?;
    println!("{:?}", john);

    // Lastly, you can delete the object.
    john.delete(&mut conn).await?;
    // You can get a single user.
    Person::get_one(1u32, &mut conn)
        .await
        .expect_err("Should not exist");

    Person {
        id: 1,
        name: "Dan".to_string(),
        age: 28,
    }
    .insert(&mut conn)
    .await?;

    let dan = Person::get_one(1u32, &mut conn).await?;
    println!("get_one {:?}", dan);

    dan.update_partial().age(29).update(&mut conn).await?;

    println!("build {:?}", dan);
    let kurt = Person::build()
        .name("Kurt".to_string())
        .age(29)
        .insert(&mut conn)
        .await?;
    println!("built {:?}", kurt);
    // // You can create a query builder.
    let people = Person::select()
        .filter("age > ?")
        .bind(50u32)
        .fetch_all(&mut conn)
        .await?;
    println!("select builder {:?}", people);

    let people = Person::query("SELECT * FROM person WHERE age > ?")
        .bind(20u32)
        .fetch_all(&mut conn)
        .await?;
    println!("raw query: {:?}", people);

    Person::insert()
        .values(vec![])
        .conflict("(id) do nothing")
        .fetch_all(&mut conn);

    Person::insert()
        .values(vec![])
        .conflict(
            "(id) set name = excluded.name\
        , limit = excluded.limit",
        )
        .fetch_all(&mut conn);

    Person::update()
        .set("age = ?")
        .bind(30u32)
        .filter("id = ?", 1)
        .execute(&mut conn);

    // the caller (fetch_all/fetch_one vs execute should modify the returning behavior.

    Ok(())
}
