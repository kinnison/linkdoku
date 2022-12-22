// @generated automatically by Diesel CLI.

diesel::table! {
    identity (uuid) {
        uuid -> Varchar,
        oidc_handle -> Varchar,
        display_name -> Varchar,
        gravatar_hash -> Varchar,
    }
}

diesel::table! {
    role (uuid) {
        uuid -> Varchar,
        owner -> Varchar,
        display_name -> Varchar,
        description -> Text,
    }
}

diesel::joinable!(role -> identity (owner));

diesel::allow_tables_to_appear_in_same_query!(
    identity,
    role,
);
