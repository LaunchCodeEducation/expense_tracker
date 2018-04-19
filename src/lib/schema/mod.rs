
    table! {
        categories (id) {
            id -> Int4,
            user_id -> Int4,
            name -> Varchar,
            descrip -> Varchar,
            archived -> Bool,
        }
    }

table! {
        expenses (id) {
            id -> Int4,
            user_id -> Int4,
            category_id -> Int4,
            created -> Varchar,
            name -> Varchar,
            amount -> Varchar,
        }
    }

table! {
        users (id) {
            id -> Int4,
            email -> Varchar,
            password -> Varchar,
        }
    }

    joinable!(categories -> users (user_id));
    joinable!(expenses -> categories (category_id));
    joinable!(expenses -> users (user_id));

    allow_tables_to_appear_in_same_query!(
        categories,
        expenses,
        users,
    );