table! {
    employees (id) {
        id -> Int4,
        firstname -> Varchar,
        lastnam -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

table! {
    reserve (id) {
        id -> Int4,
        full_name -> Varchar,
        phone_number -> Varchar,
        reserve_date -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    reserve,
);
