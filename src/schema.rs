table! {
    meetups (id) {
        id -> Int4,
        title -> Text,
        talks -> Nullable<Array<Text>>,
        date -> Timestamp,
    }
}

table! {
    proposals (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
    }
}

table! {
    talks (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        description -> Text,
        published -> Bool,
        video_link -> Nullable<Text>,
        slides_link -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    votes (id) {
        id -> Int4,
        talk_id -> Int4,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

joinable!(proposals -> users (user_id));
joinable!(talks -> users (user_id));
joinable!(votes -> talks (talk_id));
joinable!(votes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    meetups,
    proposals,
    talks,
    users,
    votes,
);
