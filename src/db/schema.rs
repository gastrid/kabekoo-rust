table! {
    use super::CheeseTypeMapping;
    use super::MilkMapping;
    use super::RindMapping;
    use super::CountryMapping;
    cheeses (id) {
        id -> Int4,
        name -> Varchar,
        photo -> Nullable<Varchar>,
        milk -> Nullable<Milk>,
        pasteurised -> Nullable<Bool>,
        cheesetype -> Nullable<CheeseTypeMapping>,
        rind -> Nullable<Rind>,
        additive -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        country -> Nullable<Country>,
        rating -> Nullable<Int4>,
        comment -> Nullable<Varchar>,
        maturity -> Nullable<Int4>,
    }
}