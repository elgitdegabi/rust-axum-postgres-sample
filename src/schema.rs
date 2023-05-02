// @generated automatically by Diesel CLI.

diesel::table! {
    user_data (user_id) {
        user_id -> BigInt,
        user_name -> Nullable<Text>,
        user_alias -> Nullable<Text>,
        user_address -> Nullable<Text>,
        user_create_ts -> Nullable<Timestamptz>,
        user_update_ts -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    vehicle (vehicle_id) {
        vehicle_id -> BigInt,
        vin -> Text,
        license_plate -> Text,
        status -> Text,
        vehicle_type -> Text,
        vehicle_date -> Timestamptz,
        vehicle_update -> Timestamptz,
    }
}
