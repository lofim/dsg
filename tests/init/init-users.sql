CREATE TABLE public.users (
    id          SERIAL primary key,
    first_name  varchar(255) NOT NULL,
    last_name   TEXT NOT NULL,
    description TEXT NOT NULL,
    deleted_at  timestamp NULL,
    role        varchar(10)
);
