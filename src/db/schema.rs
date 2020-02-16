#[derive(DbEnum, PartialEq, Debug)]
pub enum Milk {
    Cow,
    Goat,
    Sheep,
    Other
}

#[derive(DbEnum, PartialEq, Debug)]
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


#[derive(DbEnum, PartialEq, Debug)]
pub enum Rind {
    Velvety,
    Washed,
    Natural,
    Na
}


#[derive(DbEnum, PartialEq, Debug)]
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


table! {
    use super::CheeseTypeMapping;
    use super::MilkMapping;
    use super::RindMapping;
    use super::CountryMapping;
    use diesel::types::Int4;
    use diesel::types::Varchar;
    use diesel::sql_types::Bool;
    use diesel::sql_types::Nullable;
    cheeses (id) {
        id -> Int4,
        name -> Varchar,
        photo -> Nullable<Varchar>,
        milk -> MilkMapping,
        pasteurised -> Nullable<Bool>,
        cheesetype -> CheeseTypeMapping,
        rind -> RindMapping,
        additive -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        country -> CountryMapping,
        rating -> Nullable<Int4>,
        comment -> Nullable<Varchar>,
        maturity -> Nullable<Int4>,
    }
}