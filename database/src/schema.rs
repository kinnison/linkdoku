// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "visibility"))]
    pub struct Visibility;
}

diesel::table! {
    identity (uuid) {
        uuid -> Varchar,
        oidc_handle -> Varchar,
        display_name -> Varchar,
        gravatar_hash -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Visibility;

    puzzle (uuid) {
        uuid -> Varchar,
        owner -> Varchar,
        display_name -> Varchar,
        short_name -> Varchar,
        visibility -> Visibility,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Visibility;

    puzzle_state (id) {
        id -> Int4,
        puzzle -> Varchar,
        description -> Text,
        visibility -> Visibility,
        updated_at -> Timestamptz,
        data -> Text,
        uuid -> Varchar,
    }
}

diesel::table! {
    puzzle_tag (uuid) {
        uuid -> Varchar,
        puzzle -> Varchar,
        tag -> Varchar,
    }
}

diesel::table! {
    role (uuid) {
        uuid -> Varchar,
        owner -> Varchar,
        display_name -> Varchar,
        description -> Text,
        short_name -> Varchar,
    }
}

diesel::table! {
    tag (uuid) {
        uuid -> Varchar,
        name -> Varchar,
        colour -> Varchar,
        black_text -> Bool,
        description -> Varchar,
    }
}

diesel::joinable!(puzzle -> role (owner));
diesel::joinable!(puzzle_state -> puzzle (puzzle));
diesel::joinable!(puzzle_tag -> puzzle (puzzle));
diesel::joinable!(puzzle_tag -> tag (tag));
diesel::joinable!(role -> identity (owner));

diesel::allow_tables_to_appear_in_same_query!(
    identity,
    puzzle,
    puzzle_state,
    puzzle_tag,
    role,
    tag,
);
