
use super::schema::{cheeses, Milk, CheeseType, Rind, Country};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Cheese {
    pub id: i32,
    pub name: String,
    pub photo: Option<String>,
    pub milk: Milk,
    pub pasteurised: bool,
    pub cheesetype: CheeseType,
    pub rind: Rind,
    pub additive: Option<String>,
    pub region: Option<String>,
    pub country: Country,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub maturity: Option<i32>
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct FormCheese {
    pub name: String,
    pub photo: Option<String>,
    pub milk: Milk,
    pub pasteurised: bool,
    pub cheesetype: CheeseType,
    pub rind: Rind,
    pub additive: Option<String>,
    pub region: Option<String>,
    pub country: Country,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub maturity: Option<i32>
}

#[derive(Insertable)]
#[table_name = "cheeses"]
pub struct NewCheese<'a> {
    pub name: &'a str,
    pub photo: Option<String>,
    pub milk: Milk,
    pub pasteurised: bool,
    pub cheesetype: CheeseType,
    pub rind: Rind,
    pub additive: Option<String>,
    pub region: Option<String>,
    pub country: Country,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub maturity: Option<i32>

}