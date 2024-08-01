package fr.kingfight.kingManageAPI.utils;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.Objects.Status.status;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.SocketTimeoutException;
import java.net.URL;
import java.util.concurrent.CompletableFuture;


public class HttpClient {
    private KingManageAPI INSTANCE = null;
    private String BASE_URL = null;
    private Integer API_TIMEOUT = null;
    private String API_KEY = null;

    public HttpClient(KingManageAPI instance, String apiKey, String baseUrl, Integer timeout) {
        INSTANCE = instance;
        API_KEY = apiKey;
        BASE_URL = baseUrl;
        API_TIMEOUT = timeout;
    }

    public CompletableFuture<Object> sendRequest(String method, String urlString, String data) {
        return CompletableFuture.supplyAsync(() -> {
            if (!INSTANCE.getStatus().getStatus().equals(status.ENABLED)) {
                return new Problem("1", "Error", "Plugin is not enabled");
            }
            try {
                URL url = new URL(BASE_URL + urlString);
                HttpURLConnection conn = (HttpURLConnection) url.openConnection();
                conn.setRequestMethod(method);
                conn.setRequestProperty("Authorization", "Bearer " + API_KEY);
                conn.setRequestProperty("Content-Type", "application/json");
                conn.setRequestProperty("Accept", "application/json");
                conn.setRequestProperty("User-Agent", "KingManageAPI");
                conn.setConnectTimeout(API_TIMEOUT);
                conn.setReadTimeout(API_TIMEOUT);

                if (data != null) {
                    conn.setDoOutput(true);
                    conn.getOutputStream().write(data.getBytes());
                }

                int responseCode = conn.getResponseCode();
                if (responseCode < 200 || responseCode >= 300) {
                    throw new RuntimeException("HTTP Error: " + conn.getContent());
                }

                BufferedReader in = new BufferedReader(new InputStreamReader(conn.getInputStream()));
                String inputLine;
                StringBuilder response = new StringBuilder();

                while ((inputLine = in.readLine()) != null) {
                    response.append(inputLine);
                }
                in.close();
                conn.disconnect();

                return response.toString();
            } catch (SocketTimeoutException e) {
                throw new RuntimeException("TimeoutExceeded: " + e.getMessage());
            } catch (IOException e) {
                throw new RuntimeException("Unknown Error: " + e.getMessage());
            }
        }).handle((result, ex) -> {
            if (ex != null) {
                return new Problem("0", "Error", ex.getMessage());
            } else {
                return result;
            }
        });
    }
}
