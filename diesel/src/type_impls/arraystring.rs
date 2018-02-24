extern crate arrayvec;

use std::io::prelude::*;

use backend::Backend;
use deserialize::{self, FromSql};
use serialize::{self, Output, ToSql};
use sql_types::Text;

#[derive(FromSqlRow, AsExpression)]
#[diesel(foreign_derive)]
#[sql_type = "Text"]
#[allow(dead_code)]
struct ArrayStringProxy<A: arrayvec::Array<Item=u8>>(arrayvec::ArrayString<A>);

impl<DB, A> ToSql<Text, DB> for arrayvec::ArrayString<A>
where
    DB: Backend,
    A: arrayvec::Array<Item=u8>,
    str: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        self.as_str().to_sql(out)
    }
}

impl<DB, A> FromSql<Text, DB> for arrayvec::ArrayString<A>
where
    DB: Backend<RawValue=[u8]>,
    A: arrayvec::Array<Item=u8>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let str_ptr = <*const str as FromSql<Text, DB>>::from_sql(bytes)?;
        let string = unsafe { &*str_ptr };
        arrayvec::ArrayString::<A>::from(string).map_err(|e| e.into())
    }
}
