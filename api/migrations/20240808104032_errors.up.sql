CREATE TABLE logs.errors(
    error__id uuid primary key default public.gen_random_uuid(),
    content text not null,
    plugin_name text not null,
    priority smallint not null,
    created_at timestamptz not null default now()
);