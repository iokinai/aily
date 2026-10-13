// @generated automatically by Diesel CLI.

diesel::table! {
    aily_posts (id) {
        id -> Integer,
        artwork_id -> Integer,
        is_ai_generated -> Bool,
    }
}

diesel::table! {
    artworks (id) {
        id -> Integer,
        danbooru_id -> Integer,
        path -> Text,
        is_nsfw -> Bool,
    }
}

diesel::joinable!(aily_posts -> artworks (artwork_id));

diesel::allow_tables_to_appear_in_same_query!(aily_posts, artworks,);
