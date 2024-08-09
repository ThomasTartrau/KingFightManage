package fr.kingfight.kingManageAPI.events.fetched;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.utils.Helpers;
import org.json.simple.JSONObject;

import java.util.UUID;

public class Event {
    private KingManageAPI instance = KingManageAPI.getInstance();

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
                    new SendMessageE(sender, target, instance.getConfig().getString("messages.events.fetched.send_message").replace("{message}", message)).dispatch();
                } catch (Exception e) {
                    Helpers.sendAPIErrorMessage(new Exception("Error dispatching send_message event: " + e.getMessage()));
                }
                break;
            case "sanction_player":
                try {
                    UUID player_id = UUID.fromString((String) event_data.get("player_id"));
                    String staff_name = (String) event_data.get("staff_name");
                    String sanction_name = (String) event_data.get("sanction_name");
                    String sanction_type = (String) event_data.get("sanction_type");
                    Long sanction_duration = (Long) event_data.get("sanction_duration");
                    String sanction_motif = (String) event_data.get("motif");

                    if (player_id == null || staff_name == null || sanction_name == null || sanction_type == null || sanction_duration == null || sanction_motif == null) {
                        Helpers.sendAPIErrorMessage(new Exception("Missing required fields in sanction_player event"));
                        return;
                    }

                    new SanctionsE(player_id, staff_name, sanction_name, sanction_type, sanction_duration, sanction_motif).dispatch();
                } catch (Exception e) {
                    Helpers.sendAPIErrorMessage(new Exception("Error dispatching sanction_player event: " + e.getMessage()));
                }
                break;
            default:
                Helpers.sendAPIErrorMessage(new Exception("Unknown event type: " + event_type));
                break;
        }
    }
}
