// @generated automatically by Diesel CLI.

diesel::table! {
    identity (id) {
        id -> Int4,
        oidc_handle -> Varchar,
        display_name -> Varchar,
        gravatar_hash -> Varchar,
    }
}
