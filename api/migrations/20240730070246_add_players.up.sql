CREATE SCHEMA players;

CREATE TABLE players.player (
    player__id uuid primary key,
    name text not null,
    created_at timestamptz not null default now()
);

CREATE TABLE players.logged_in (
    logged_in__id uuid primary key,
    player__id uuid not null,
    created_at timestamptz not null default now(),
    CONSTRAINT fk_logged_in__player__id FOREIGN KEY (player__id) REFERENCES players.player(player__id)
);

CREATE TABLE players.logged_out (
    logged_out__id uuid primary key,
    player__id uuid not null,
    created_at timestamptz not null default now(),
    CONSTRAINT fk_logged_out__player__id FOREIGN KEY (player__id) REFERENCES players.player(player__id)
);

CREATE FUNCTION is_logged_in(player__id uuid) RETURNS boolean AS $$
DECLARE
    last_login timestamptz;
    last_logout timestamptz;
BEGIN
    SELECT created_at INTO last_login
    FROM players.logged_in
    WHERE player__id = is_logged_in.player__id
    ORDER BY created_at DESC
    LIMIT 1;

    SELECT created_at INTO last_logout
    FROM players.logged_out
    WHERE player__id = is_logged_in.player__id
    ORDER BY created_at DESC
    LIMIT 1;

    IF last_login IS NULL THEN
        RETURN FALSE;
    ELSIF last_logout IS NULL THEN
        RETURN TRUE;
    ELSE
        RETURN last_login > last_logout;
    END IF;
END;
$$ LANGUAGE plpgsql;