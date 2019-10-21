table! {
    domains (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    employees (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    machines (id) {
        id -> Int8,
        domain_id -> Nullable<Int8>,
        employee_id -> Nullable<Int8>,
        host -> Varchar,
    }
}

table! {
    machines_programs (id) {
        id -> Int8,
        machine_id -> Int8,
        program_id -> Int8,
        sw_report_id -> Int8,
        path -> Nullable<Varchar>,
    }
}

table! {
    programs (id) {
        id -> Int8,
        publisher_id -> Nullable<Int8>,
        name -> Varchar,
        version -> Nullable<Varchar>,
    }
}

table! {
    publishers (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    sw_reports (id) {
        id -> Int8,
        generation_date -> Date,
    }
}

joinable!(machines -> domains (domain_id));
joinable!(machines -> employees (employee_id));
joinable!(machines_programs -> machines (machine_id));
joinable!(machines_programs -> programs (program_id));
joinable!(machines_programs -> sw_reports (sw_report_id));
joinable!(programs -> publishers (publisher_id));

allow_tables_to_appear_in_same_query!(
    domains,
    employees,
    machines,
    machines_programs,
    programs,
    publishers,
    sw_reports,
);
