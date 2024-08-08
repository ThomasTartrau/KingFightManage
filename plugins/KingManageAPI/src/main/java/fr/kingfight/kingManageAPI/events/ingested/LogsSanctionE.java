package fr.kingfight.kingManageAPI.events.ingested;

import fr.kingfight.kingManageAPI.api.events.ingested.LogsSanctionAPI;
import org.json.simple.JSONObject;

import java.util.UUID;
import java.util.concurrent.CompletableFuture;

public abstract class LogsSanctionE implements LogsSanctionAPI {

    @Override
    public CompletableFuture<Object> dispatch(UUID player_id, String staff_name, UUID sanction_id, String motif, Boolean generate_event, String sanction_type) {
        JSONObject data = new JSONObject();
        data.put("player_id", player_id.toString());
        data.put("staff_name", staff_name);
        data.put("sanction_id", sanction_id.toString());
        data.put("motif", motif);
        data.put("generate_event", generate_event);

        return new PostEvent("POST", "/sanctions/" + sanction_type, data.toJSONString()).sendRequest();
    }
}
