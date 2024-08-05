package fr.kingfight.kingManageAPI.tasks;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.Objects.Status.status;
import fr.kingfight.kingManageAPI.events.fetched.Event;
import fr.kingfight.kingManageAPI.utils.Helpers;
import fr.kingfight.kingManageAPI.utils.Problem;
import org.bukkit.scheduler.BukkitRunnable;
import org.json.simple.JSONArray;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;

import java.util.ArrayList;
import java.util.UUID;
import java.util.concurrent.CompletableFuture;

public class FetchEventsTask extends BukkitRunnable {
    private final KingManageAPI instance;

    public FetchEventsTask(KingManageAPI K) {
        this.instance = K;
    }

    @Override
    public void run() {
        if (!instance.getStatus().getStatus().equals(status.ENABLED)) return;

        CompletableFuture<String> futureEvents = fetchEvents();
        futureEvents.handle((eventsString, throwable) -> {
            if (throwable != null) {
                Helpers.sendAPIErrorMessage(new Exception("FetchEvent Error: " + throwable.getMessage()));
                return null;
            }

            if (eventsString == null || eventsString.isEmpty()) return null;

            JSONParser parser = new JSONParser();
            try {
                JSONObject jsonObject = (JSONObject) parser.parse(eventsString);
                if (jsonObject.containsKey("events")) {
                    JSONArray events = (JSONArray) jsonObject.get("events");
                    if (!events.isEmpty()) {
                        for (Object evenstObj : events) {
                            JSONObject eventObj = (JSONObject) evenstObj;
                            UUID eventId = UUID.fromString((String) eventObj.get("event_id"));
                            String eventType = (String) eventObj.get("event_type");
                            JSONObject eventData = (JSONObject) eventObj.get("event_data");

                            new Event(eventId, eventType, eventData).dispatch();
                        }
                    }
                } else {
                    Helpers.sendAPIErrorMessage(new Exception("FetchEvent Error: No events key found"));
                }
            } catch (Exception e) {
                Helpers.sendAPIErrorMessage(new Exception("FetchEvent Error: " + e.getMessage()));
            }

            return null;
        });
    }

    private CompletableFuture<String> fetchEvents() {

        CompletableFuture<Object> response = instance.sendRequest("GET", "/events", null).handle((result, ex) -> {
            if (ex != null) {
                Helpers.sendAPIErrorMessage(new Exception("FetchEvent Error: " + ex.getMessage()));
                return new ArrayList<>();
            }
            return result;
        });


        return response.thenApply(result -> {
            if (result == null) {
                Helpers.sendAPIErrorMessage(new Exception("FetchEvent Error: Null result"));
                return null;
            } else if (result instanceof Problem) {
                Problem problem = (Problem) result;
                Helpers.sendAPIErrorMessage(new Exception("FetchEvent Error: " + problem.getProblem()));
                return null;
            } else if (result instanceof String) {
                return result.toString();
            } else {
                Helpers.sendAPIErrorMessage(new Exception("FetchEvent Error: Unknown result type"));
                return null;
            }
        });
    }
}