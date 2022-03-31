table! {
    devices (id) {
        id -> Int4,
        name -> Varchar,
        consumption -> Float4,
        duration -> Nullable<Int4>,
    }
}

table! {
    meterdevices (id) {
        id -> Int4,
        meter -> Int4,
        device -> Int4,
        turned_on -> Bool,
        duration -> Nullable<Int4>,
    }
}

table! {
    meters (id) {
        id -> Int4,
        occupants -> Int4,
        day_consumption -> Float4,
        night_consumption -> Float4,
        last_snapshot -> Timestamp,
        latitude -> Float4,
        longitude -> Float4,
    }
}

joinable!(meterdevices -> devices (device));
joinable!(meterdevices -> meters (meter));

allow_tables_to_appear_in_same_query!(
    devices,
    meterdevices,
    meters,
);
