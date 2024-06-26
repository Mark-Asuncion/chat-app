use sqlx::postgres::PgPoolOptions;

use crate::{database::{query::{ builder::QueryBuilder, QueryValue, filter::Filter, ORDER, join }, init, schema::{account::Account, salt::Salt}, DatabaseUtils}, utils::password::Password};

#[test]
fn _t_select() {
    use QueryValue::{ Varchar, Int };
    let mut qb = QueryBuilder::new();
    qb.select("table", Some(vec!["asd", "qwe"]))
        .filter( Filter::If("lvalue".into(), "=".into(), Varchar("asd".into())) )
        .or()
        .filter( Filter::If("lvalue".into(), "=".into(), Varchar("a".into())) )
        .filter( Filter::If("lvalue".into(), "=".into(), Int(-32)) )
        .order_by("asd", ORDER::DESC);
    println!("{}", qb.build());
}

#[test]
fn _t_update() {
    use QueryValue::{ Varchar, Int };
    let mut qb = QueryBuilder::new();
    qb.update("table")
        .set(("key".into(), Varchar("value".into())))
        .set(("key2".into(), Int(542)))
        .filter( Filter::If("lvalue".into(), "=".into(), Varchar("a".into())) );
    println!("{}", qb.build());
}

#[test]
fn _t_insert() {
    use QueryValue::{ Varchar, Int };
    let mut qb = QueryBuilder::new();
    let keys = vec!["hello", "asdcn"];
    let values = vec![
        vec![Varchar(String::from("hello")), Varchar(String::from("asdcn"))],
        vec![Int(32), Varchar(String::from("asdcn"))]
    ];
    qb.insert("table", keys)
        .value(values.get(0).unwrap().to_vec())
        .value(values.get(1).unwrap().to_vec());

    println!("{}", qb.build());
}

#[test]
fn _t_join() {
    let mut qb = QueryBuilder::new();
    let mut cols = Account::as_columns();
    let mut salt_col = Salt::as_columns();
    cols.append(&mut salt_col);
    qb.select(Account::table(), Some(cols))
        .filter(Filter::if_from("username", "=", QueryValue::Varchar("user".into())))
    .join(join::Join::inner(Salt::table(), (Account::pkey(), Salt::as_columns().get(1).unwrap())));

    println!("{}", qb.build());
}

#[test]
fn _t_hasher() {
    let pass = "hello123";
    let password = Password::hash(pass);
    dbg!(&password, password.hash.len());

    let password2 = Password::hash_from(&password.salt, pass);
    dbg!(&password2, password2.hash.len());

    assert!(password2.verify(&password.hash));
}
