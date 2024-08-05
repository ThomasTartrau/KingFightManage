package fr.kingfight.kingManageAPI.events.ingested;

import java.util.UUID;
import java.util.concurrent.CompletableFuture;

import org.json.simple.JSONObject;

public class PlayerJoinE {
    
    @SuppressWarnings("unchecked")
    public static CompletableFuture<Object> dispatch(UUID playerUuid, String playerName) {
        JSONObject data = new JSONObject();
        data.put("player_id", playerUuid.toString());
        data.put("username", playerName);

        return new PostEvent("POST", "/players/join", data.toJSONString()).sendRequest();
    }
}
