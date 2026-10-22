// @generated automatically by Diesel CLI.

diesel::table! {
    aily_posts (id) {
        id -> Integer,
        artwork_id -> Integer,
        is_ai_generated -> Bool,
    }
}

diesel::table! {
    artwork_tag (id) {
        id -> Integer,
        artwork_id -> Integer,
        tag_id -> Integer,
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

diesel::table! {
    tags (id) {
        id -> Integer,
        tag_name -> Text,
    }
}

diesel::joinable!(aily_posts -> artworks (artwork_id));
diesel::joinable!(artwork_tag -> artworks (artwork_id));
diesel::joinable!(artwork_tag -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(aily_posts, artwork_tag, artworks, tags,);
