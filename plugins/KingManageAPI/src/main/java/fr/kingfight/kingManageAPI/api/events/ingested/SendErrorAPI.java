package fr.kingfight.kingManageAPI.api.events.ingested;

import java.util.concurrent.CompletableFuture;

public interface SendErrorAPI {
    /**
     * Send an error log to the API
     * @param content any string format (json, string, xml, etc...)
     * @param plugin_name
     * @param priority 0 = low, 1 = medium, 2 = high, 3 = critical
     * @return CompletableFuture
     *      (Object type String = null if success)
     *      (Object type Boolean = false if error)
     */
    CompletableFuture<Object> dispatch(String content, String plugin_name, Integer priority);
}
