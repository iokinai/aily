diesel::table! {
    #[allow(non_snake_case)]
    Arts(id) {
        id -> Integer,
        danbooru_id -> Nullable<Integer>,
        is_nsfw -> Bool,
        is_ai_generated -> Bool,
        is_ai_assisted -> Bool,
        path -> VarChar
    }
}
