table! {
    employees (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

table! {
    teachers (email) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        designation -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    teachers,
);
