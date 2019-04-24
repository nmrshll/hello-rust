// #[macro_use] extern crate diesel_codegen;

// embed_migrations!();

table! {
    laptops (id) {
        id -> Int4,
        company -> Varchar,
        product -> Varchar,
        typename -> Nullable<Varchar>,
        inches -> Nullable<Varchar>,
        screenresolution -> Nullable<Varchar>,
        cpu -> Nullable<Varchar>,
        ram -> Nullable<Varchar>,
        memory -> Nullable<Varchar>,
        gpu -> Nullable<Varchar>,
        opsys -> Nullable<Varchar>,
        weight -> Nullable<Varchar>,
        price_euros -> Nullable<Float4>,
    }
}
