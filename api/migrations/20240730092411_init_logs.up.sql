CREATE SCHEMA logs;

CREATE TABLE logs.log (
    log__id uuid primary key default public.gen_random_uuid(),
    username text not null,
    action text not null,
    created_at timestamptz not null default now()
);

CREATE TABLE logs.staffs (
    staff_log__id uuid primary key default public.gen_random_uuid(),
    username text not null,
    action text not null,
    created_at timestamptz not null default now()
);

CREATE TABLE logs.boutique (
    boutique_log__id uuid primary key default public.gen_random_uuid(),
    username text not null,
    action text not null,
    created_at timestamptz not null default now()
);

CREATE TABLE logs.pb (
    pb_log__id uuid primary key default public.gen_random_uuid(),
    username text not null,
    action text not null,
    amount smallint not null,
    created_at timestamptz not null default now()
);