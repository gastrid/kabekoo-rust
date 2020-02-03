

#[derive(Queryable)]
pub struct Cheese {
    pub id: i32,
    pub name: String,
    pub photo: Option(String),
    pub milk: Milk,
    pub pasteurised: Boolean,
    pub cheesetype: CheeseType,
    pub rind: Rind,
    pub additive: Option(String),
    pub region: Option(String),
    pub country: Country,
    pub rating: i32,
    pub comment: Option(String),
    pub maturity: i32
}