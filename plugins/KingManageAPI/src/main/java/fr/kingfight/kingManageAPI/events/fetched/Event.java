package fr.kingfight.kingManageAPI.events.fetched;

import fr.kingfight.kingManageAPI.utils.Helpers;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;

import java.util.UUID;

public class Event {
    private UUID event_id;
    private String event_type;
    private JSONObject event_data;

    public Event(UUID event_id, String event_type, JSONObject event_data) {
        this.event_id = event_id;
        this.event_type = event_type;
        this.event_data = event_data;
    }

    public void dispatch() {
        switch (event_type) {
            case "send_message":
                try {
                    String sender = (String) event_data.get("sender");
                    UUID target = UUID.fromString((String) event_data.get("target"));
                    String message = (String) event_data.get("message");

                    if (sender == null || message == null) {
                        Helpers.sendAPIErrorMessage(new Exception("Missing required fields in send_message event"));
                        return;
                    }
                    new SendMessage(sender, target, message).dispatch();
                } catch (Exception e) {
                    Helpers.sendAPIErrorMessage(new Exception("Error dispatching send_message event: " + e.getMessage()));
                }
                break;
            default:
                Helpers.sendAPIErrorMessage(new Exception("Unknown event type: " + event_type));
                break;
        }
    }
}
