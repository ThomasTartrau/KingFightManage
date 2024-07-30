CREATE SCHEMA players;

CREATE TABLE players.player (
    player__id uuid primary key,
    name text not null,
    created_at timestamptz not null default now()
);

CREATE TABLE players.logged_in (
    logged_in__id uuid primary key default public.gen_random_uuid(),
    player_id uuid not null,
    created_at timestamptz not null default now(),
    CONSTRAINT fk_logged_in__player__id FOREIGN KEY (player_id) REFERENCES players.player(player__id)
);


CREATE TABLE players.logged_out (
    logged_out__id uuid primary key default public.gen_random_uuid(),
    player_id uuid not null,
    created_at timestamptz not null default now(),
    CONSTRAINT fk_logged_out__player__id FOREIGN KEY (player_id) REFERENCES players.player(player__id)
);

CREATE FUNCTION players.is_logged_in(player__id uuid) RETURNS boolean AS $$
DECLARE
    last_login timestamptz;
    last_logout timestamptz;
BEGIN
    SELECT created_at INTO last_login
    FROM players.logged_in
    WHERE player_id = player__id
    ORDER BY created_at DESC
    LIMIT 1;

    SELECT created_at INTO last_logout
    FROM players.logged_out
    WHERE player_id = player__id
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

CREATE FUNCTION players.get_online_players() RETURNS TABLE (player__id uuid, name text) AS $$
    SELECT player__id, name
    FROM players.player
    WHERE players.is_logged_in(player__id) = TRUE;
$$ LANGUAGE sql;