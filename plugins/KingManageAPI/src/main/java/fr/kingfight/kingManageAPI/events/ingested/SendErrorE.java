package fr.kingfight.kingManageAPI.events.ingested;

import fr.kingfight.kingManageAPI.api.events.ingested.SendErrorAPI;
import org.json.simple.JSONObject;

import java.util.concurrent.CompletableFuture;

public abstract class SendErrorE implements SendErrorAPI {

    @Override
    public CompletableFuture<Object> dispatch(String content, String plugin_name, Integer priority) {
        JSONObject data = new JSONObject();
        data.put("content", content);
        data.put("plugin_name", plugin_name);
        data.put("priority", priority);

        return new PostEvent("POST", "/errors", data.toJSONString()).sendRequest();
    }
}
