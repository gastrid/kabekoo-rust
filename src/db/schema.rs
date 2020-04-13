// table! {
//     use super::MilkMapping;
//     use diesel::types::Int4;
//     use diesel::types::Varchar;
//     use diesel::sql_types::Bool;
//     use diesel::sql_types::Nullable;
//     cheeses (id) {
//         id -> Int4,
//         name -> Varchar,
//         photo -> Nullable<Varchar>,
//         milk -> MilkMapping,
//         pasteurised -> Bool,
//     }
// }


#[derive(DbEnum, Serialize, Deserialize, Clone, PartialEq, Debug, FromFormValue)]
pub enum Milk {
    Cow,
    Goat,
    Sheep,
    Other
}

#[derive(DbEnum, Serialize, Deserialize, Clone, PartialEq, Debug, FromFormValue)]
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


#[derive(DbEnum, Serialize, Deserialize, Clone, PartialEq, Debug, FromFormValue)]
pub enum Rind {
    Velvety,
    Washed,
    Natural,
    Na
}


#[derive(DbEnum, Serialize, Deserialize, Clone, PartialEq, Debug, FromFormValue)]
pub enum Country {
    France,
    England,
    Italy,
    Switzerland,
    Wales,
    Spain,
    Scotland,
    Ireland,
    Other
}

pub trait EnumToPrint {
    fn to_print(&self) -> &str;
}

impl EnumToPrint for Milk {
    fn to_print(&self) -> &str {
        match self {
            Milk::Cow => "cow",
            Milk::Goat => "goat",
            Milk::Sheep => "sheep",
            Milk::Other => "other"
        }
    }
}

impl EnumToPrint for CheeseType {
    fn to_print(&self) -> &str {
        match self {
            CheeseType::Fresh => "fresh",
            CheeseType::Soft => "soft",
            CheeseType::ColdPressed => "coldPressed",
            CheeseType::HotPressed => "hotPressed",
            CheeseType::Hard => "hard",
            CheeseType::Blue => "blue",
            CheeseType::PastaFilata => "pastaFilata",
            CheeseType::Other => "other"
        }
    }
}

impl EnumToPrint for Rind {
    fn to_print(&self) -> &str {
        match self {
            Rind::Velvety => "velvety",
            Rind::Washed => "washed",
            Rind::Natural => "natural",
            Rind::Na => "NA"
        }
    }
}

impl EnumToPrint for Country {
    fn to_print(&self) -> &str {
        match self {
            Country::France => "france",
            Country::England => "england",
            Country::Italy => "italy",
            Country::Switzerland => "switzerland",
            Country::Wales => "wales",
            Country::Spain => "spain",
            Country::Scotland => "scotland",
            Country::Ireland => "ireland",
            Country::Other => "other"
        }
    }
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
        pasteurised -> Bool,
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

