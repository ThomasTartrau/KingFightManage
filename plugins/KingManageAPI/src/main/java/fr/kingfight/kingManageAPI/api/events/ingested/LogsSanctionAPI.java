package fr.kingfight.kingManageAPI.api.events.ingested;

import java.util.UUID;
import java.util.concurrent.CompletableFuture;

public interface LogsSanctionAPI {
    CompletableFuture<Object> dispatch(UUID player_id, String staff_name, UUID sanction_id, String motif, Boolean generate_event, String sanction_type);
}
