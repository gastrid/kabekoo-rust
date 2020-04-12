use diesel;

#[database("kabekoo")]
pub struct CheesesDbConn(diesel::PgConnection);
