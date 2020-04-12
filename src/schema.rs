table! {
    cheeses (id) {
        id -> Int4,
        name -> Text,
        photo -> Nullable<Varchar>,
        milk -> Milk,
        pasteurised -> Nullable<Bool>,
        cheesetype -> Cheese_type,
        rind -> Rind,
        additive -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        country -> Nullable<Country>,
        rating -> Nullable<Int4>,
        comment -> Nullable<Varchar>,
        maturity -> Nullable<Int4>,
    }
}
