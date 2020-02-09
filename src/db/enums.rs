#[derive(DbEnum)]
pub enum Milk {
    Cow,
    Goat,
    Sheep,
    Other
}
#[derive(DbEnum)]
pub enum CheeseType {
    Fresh,
    Soft,
    ColdPressed,
    HotPressed,
    Hard,
    Blue,
    PastaFilata,
    Other
}
#[derive(DbEnum)]
pub enum Rind {
    Velvety,
    Washed,
    Natural,
    Na
}
#[derive(DbEnum)]
pub enum Country {
    Fransce,
    England,
    Italy,
    Switzerland,
    Wales,
    Spain,
    Scotland,
    Ireland,
    Other
}
