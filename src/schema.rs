table! {
    domains (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    machines (id) {
        id -> Int4,
        host -> Varchar,
        domain_id -> Nullable<Int4>,
    }
}

table! {
    programs (id) {
        id -> Int4,
        publisher_id -> Nullable<Int4>,
        name -> Varchar,
        version -> Varchar,
    }
}

table! {
    publishers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users_programs (id) {
        id -> Int4,
        machine_id -> Int4,
        program_id -> Int4,
        date_installed -> Nullable<Date>,
        path -> Nullable<Varchar>,
    }
}

joinable!(machines -> domains (domain_id));
joinable!(programs -> publishers (publisher_id));
joinable!(users_programs -> machines (machine_id));
joinable!(users_programs -> programs (program_id));

allow_tables_to_appear_in_same_query!(
    domains,
    machines,
    programs,
    publishers,
    users_programs,
);
