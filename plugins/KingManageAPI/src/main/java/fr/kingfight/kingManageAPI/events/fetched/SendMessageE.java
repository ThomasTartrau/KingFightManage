package fr.kingfight.kingManageAPI.events.fetched;

import org.bukkit.Bukkit;
import org.bukkit.entity.Player;

import java.util.UUID;

public class SendMessageE {
    private String sender;
    private UUID target;
    private String message;

    public SendMessageE(String sender, UUID target, String message) {
        this.sender = sender;
        this.target = target;
        this.message = message;
    }

    public void dispatch() {
        Player player = Bukkit.getPlayer(target);
        if (player == null) return;
        player.sendMessage(message.replace("{sender}", sender));
    }
}
