#[macro_use]
extern crate diesel;

use diesel::*;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

fn main() {
    use self::users::dsl::*;

    let foo = "foo".into_sql::<types::VarChar>();
    let command = update(users).set(foo.eq(name));
    //~^ ERROR Column
}
