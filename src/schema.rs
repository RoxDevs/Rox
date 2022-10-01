// @generated automatically by Diesel CLI.

diesel::table! {
    pkgs (id) {
        id -> Nullable<Integer>,
        version -> Text,
        name -> Text,
        path -> Text,
        repo_url -> Text,
    }
}
