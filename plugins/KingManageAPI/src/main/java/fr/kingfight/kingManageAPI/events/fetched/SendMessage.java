package fr.kingfight.kingManageAPI.events.fetched;

import org.bukkit.Bukkit;
import org.bukkit.entity.Player;

import java.util.UUID;

public class SendMessage {
    private String sender;
    private UUID target;
    private String message;

    public SendMessage(String sender, UUID target, String message) {
        this.sender = sender;
        this.target = target;
        this.message = message;
    }

    public void dispatch() {
        Player player = Bukkit.getPlayer(target);
        System.out.println(message);
        if (player == null) return;
        player.sendMessage(message.replace("{sender}", sender));
    }
}
