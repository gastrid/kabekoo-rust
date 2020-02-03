table! {
    cheeses (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        id -> Integer,
        name -> Text,
        photo -> Nullable<Text),
        milk -> Milk,
        pasteurised -> Boolean,
        cheesetype -> CheeseType,
        rind -> Rind,
        additive -> Nullable<Text),
        region -> Nullable<Text),
        country -> Country,
        rating -> Integer,
        comment -> Nullable<Text),
        maturity -> Integer
    }
}