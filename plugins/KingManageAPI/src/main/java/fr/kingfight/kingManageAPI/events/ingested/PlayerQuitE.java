package fr.kingfight.kingManageAPI.events.ingested;

import org.json.simple.JSONObject;

import java.util.UUID;
import java.util.concurrent.CompletableFuture;

public class PlayerQuitE {
    @SuppressWarnings("unchecked")
    public static CompletableFuture<Object> dispatch(UUID playerUuid) {
        JSONObject data = new JSONObject();
        data.put("player_id", playerUuid.toString());

        return new PostEvent("POST", "/players/leave", data.toJSONString()).sendRequest();
    }
}
