package fr.kingfight.kingManageAPI.api;

import fr.kingfight.kingManageAPI.utils.Problem;

import java.util.concurrent.CompletableFuture;

public interface HttpClientAPI {
    /**
     * Send a request to the API
     * @param method The HTTP method
     *               (GET, POST, PUT, DELETE)
     * @param urlString The URL to send the request
     *                  (e.g. /staffs)
     * @param data The data to send
     *             (e.g. {"name": "John"})
     *             (null if no data to send)
     *             (only for POST and PUT methods)
     *             (optional)
     * @return CompletableFuture
     *        (Object if success)
     *        (Problem if error)
     */
    CompletableFuture<Object> sendRequest(String method, String urlString, String data);

    /**
     * Print a problem to the console
     * @param problem The problem to print
     */
    void printProblem(Problem problem);
}
