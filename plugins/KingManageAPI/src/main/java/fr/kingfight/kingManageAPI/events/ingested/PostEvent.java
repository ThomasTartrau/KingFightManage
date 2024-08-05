package fr.kingfight.kingManageAPI.events.ingested;

import java.util.concurrent.CompletableFuture;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.Objects.Status.status;
import fr.kingfight.kingManageAPI.utils.Helpers;
import fr.kingfight.kingManageAPI.utils.Problem;

public class PostEvent {
    private String method;
    private String urlString;
    private String data;

    public PostEvent(String method, String urlString, String data) {
        this.method = method;
        this.urlString = urlString;
        this.data = data;
    }

    public CompletableFuture<Object> sendRequest() {
        KingManageAPI instance = KingManageAPI.getInstance();
        if (!instance.getStatus().getStatus().equals(status.ENABLED)) return CompletableFuture.failedFuture(new Exception("API is disabled"));

        CompletableFuture<Object> future = instance.sendRequest(method, urlString, data);
        future.thenAccept((result) -> {
            if (result instanceof Problem) {
                Problem problem = (Problem) result;
                Helpers.sendAPIErrorMessage(new Exception("PostEvent Error: " + problem.getProblem()));
            }
        });

        return future.thenApply(result -> {
            if (result instanceof Problem) {
                return Boolean.FALSE;
            } else {
                return result;
            }
        });
    }
}
