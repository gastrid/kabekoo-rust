#[derive(DbEnum, Debug)]
pub enum Milk {
    Cow,
    Goat,
    Sheep,
    Other
}

#[derive(DbEnum, Debug)]
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


#[derive(DbEnum, Debug)]
pub enum Rind {
    Velvety,
    Washed,
    Natural,
    Na
}


#[derive(DbEnum, Debug)]
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
    cheeses (id) {
        id -> Int4,
        name -> Varchar,
        photo -> Nullable<Varchar>,
        milk -> Nullable<MilkMapping>,
        pasteurised -> Nullable<Bool>,
        cheesetype -> Nullable<CheeseTypeMapping>,
        rind -> Nullable<RindMapping>,
        additive -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        country -> Nullable<CountryMapping>,
        rating -> Nullable<Int4>,
        comment -> Nullable<Varchar>,
        maturity -> Nullable<Int4>,
    }
}