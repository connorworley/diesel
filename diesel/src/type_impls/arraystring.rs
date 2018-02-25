extern crate arrayvec;

use std::io::prelude::*;

use backend::Backend;
use deserialize::{self, FromSql};
use serialize::{self, Output, ToSql};
use sql_types::Varchar;

#[derive(FromSqlRow, AsExpression)]
#[diesel(foreign_derive)]
#[sql_type = "Varchar"]
#[allow(dead_code)]
struct ArrayStringProxy<A: arrayvec::Array<Item=u8>>(arrayvec::ArrayString<A>);

impl<DB, A> ToSql<Varchar, DB> for arrayvec::ArrayString<A>
where
    DB: Backend,
    A: arrayvec::Array<Item=u8>,
    str: ToSql<Varchar, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        self.as_str().to_sql(out)
    }
}

impl<DB, A> FromSql<Varchar, DB> for arrayvec::ArrayString<A>
where
    DB: Backend<RawValue=[u8]>,
    A: arrayvec::Array<Item=u8>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let str_ptr = <*const str as FromSql<Varchar, DB>>::from_sql(bytes)?;
        let string = unsafe { &*str_ptr };
        arrayvec::ArrayString::<A>::from(string).map_err(|e| e.into())
    }
}

#[test]
#[cfg(feature = "mysql")]
fn arraystring_to_sql() {
    use mysql::Mysql;
    let mut bytes = Output::test();
    let test_arraystring = arrayvec::ArrayString::<[u8; 32]>::from("abcdefghijklmnopqrstuvwxyz012345").unwrap();
    ToSql::<Varchar, Mysql>::to_sql(&test_arraystring, &mut bytes).unwrap();
    assert_eq!(bytes, test_arraystring.as_bytes());
}

#[test]
#[cfg(feature = "mysql")]
fn some_arraystring_from_sql() {
    use mysql::Mysql;
    let input_arraystring = arrayvec::ArrayString::<[u8; 32]>::from("abcdefghijklmnopqrstuvwxyz012345").unwrap();
    let output_arraystring: arrayvec::ArrayString<[u8; 32]> = FromSql::<Varchar, Mysql>::from_sql(Some(input_arraystring.as_bytes())).unwrap();
    assert_eq!(input_arraystring, output_arraystring);
}

#[test]
#[cfg(feature = "mysql")]
fn bad_arraystring_from_sql() {
    use mysql::Mysql;
    let arraystring = <arrayvec::ArrayString<[u8; 8]> as FromSql<Varchar, Mysql>>::from_sql(Some(b"imoutooooo"));
    assert_eq!(arraystring.unwrap_err().description(), "insufficient capacity");
}

#[test]
#[cfg(feature = "mysql")]
fn no_arraystring_from_sql() {
    use mysql::Mysql;
    let arraystring = <arrayvec::ArrayString<[u8; 8]> as FromSql<Varchar, Mysql>>::from_sql(None);
    assert_eq!(
        arraystring.unwrap_err().description(),
        "Unexpected null for non-null column"
    );
}
