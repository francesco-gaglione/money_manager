use crate::schema::account;
use crate::schema::category;
use crate::schema::money_transaction;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub account_type: String,
    pub initial_balance: f32,
}

impl AsRef<str> for Account {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub account_type: &'a str,
    pub initial_balance: f32,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub is_income: bool
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Insertable)]
#[diesel(table_name = category)]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub is_income: bool
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::money_transaction)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MoneyTransaction {
    pub id: i32,
    pub bank_account: i32, // Foreign key referencing the 'account' table
    pub transaction_category: i32, // Foreign key referencing the 'category' table
    pub description: String,
    pub amount: f32,
    pub transaction_date: chrono::NaiveDateTime,
    pub is_expense: bool,
}

#[derive(Insertable)]
#[diesel(table_name = money_transaction)]
pub struct NewMoneyTransaction {
    pub bank_account: i32,
    pub transaction_category: i32,
    pub description: String,
    pub amount: f32,
    pub transaction_date: chrono::NaiveDateTime,
    pub is_expense: bool,
}