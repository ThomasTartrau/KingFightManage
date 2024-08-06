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
    user__id uuid not null,
    created_at timestamptz not null default now(),
    constraint log_sanction__id_fk foreign key (sanction__id) references sanctions.sanction (sanction__id) on delete cascade on update cascade,
    constraint log_user__id_fk foreign key (user__id) references iam.user (user__id) on delete cascade on update cascade
);