CREATE SCHEMA events;

CREATE TABLE events.event (
    event__id uuid primary key default public.gen_random_uuid(),
    event_type text not null,
    event_data jsonb NOT NULL,
    created_at timestamptz not null default now(),
    dispatched_at timestamptz,
    status text not null default 'pending'
);