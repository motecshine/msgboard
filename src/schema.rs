table! {
    posts (id) {
        id -> Integer,
        content -> Text,
        #[sql_name = "type"]
        type_ -> Integer,
        agree -> Tinyint,
        disagree -> Tinyint,
        created_at -> Bigint,
    }
}
