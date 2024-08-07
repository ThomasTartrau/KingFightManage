create schema sanctions;

create table sanctions.sanction(
    sanction__id uuid primary key default public.gen_random_uuid(),
    type text not null,
    name text not null,
    motif text not null,
    duration bigint not null,
    created_at timestamptz not null default now()
);

create table logs.sanction(
    log__id uuid primary key default public.gen_random_uuid(),
    sanction__id uuid not null,
    player__id uuid not null,
    staff_name text not null,
    created_at timestamptz not null default now(),
    constraint log_sanction__id_fk foreign key (sanction__id) references sanctions.sanction (sanction__id) on delete cascade on update cascade,
    constraint log_player_id_fk foreign key (player__id) references players.player (player__id) on delete cascade on update cascade
);