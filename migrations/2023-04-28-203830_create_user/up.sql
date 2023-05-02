-- Your SQL goes here
create table if not exists user_data(
	user_id bigint generated always as identity PRIMARY KEY,
	user_name text NOT NULL,
	user_alias text NOT NULL,
	user_address text NOT NULL,
	user_create_ts timestamp with time zone NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC'),
	user_update_ts timestamp with time zone NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC')
);
create table if not exists vehicle(
	vehicle_id bigint generated always as identity PRIMARY KEY,
	vin text NOT NULL,
	license_plate text NOT NULL,
	status text NOT NULL,
	vehicle_type text NOT NULL,
	vehicle_date timestamp with time zone NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC'),
	vehicle_update timestamp with time zone NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC')
);