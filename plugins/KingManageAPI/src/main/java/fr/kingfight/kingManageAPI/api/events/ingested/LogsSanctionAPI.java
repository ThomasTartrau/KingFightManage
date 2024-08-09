package fr.kingfight.kingManageAPI.api.events.ingested;

import java.util.UUID;
import java.util.concurrent.CompletableFuture;

public interface LogsSanctionAPI {
    /**
     * Send a log sanction to the API, this dispatch event won't create a sanction event
     * @param player_id
     * @param staff_name
     * @param sanction_id The sanction id get at the load of the sanction plugin
     * @param motif
     * @param generate_event Put to false to not generate an sanction event
     * @param sanction_type [mute, kick, ban]
     * @return CompletableFutur
     *       (Object type String = null if success)
     *       (Object type Boolean = false if error)
     */
    CompletableFuture<Object> dispatch(UUID player_id, String staff_name, UUID sanction_id, String motif, Boolean generate_event, String sanction_type);
}
