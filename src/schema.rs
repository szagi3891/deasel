table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        counter -> Int4,
    }
}

table! {
    posts2 (id) {
        id -> Int4,
        title2 -> Varchar,
        body2 -> Text,
        published2 -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    posts2,
);
