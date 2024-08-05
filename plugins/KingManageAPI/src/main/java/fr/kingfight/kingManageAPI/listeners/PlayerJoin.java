package fr.kingfight.kingManageAPI.listeners;

import fr.kingfight.kingManageAPI.KingManageAPI;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerJoinEvent;

public class PlayerJoin implements Listener {
    private final KingManageAPI instance;

    public PlayerJoin() {
        instance = KingManageAPI.getInstance();
    }

    @EventHandler
    public void onPlayerJoin(PlayerJoinEvent event) {
        fr.kingfight.kingManageAPI.events.ingested.PlayerJoinE.dispatch(event.getPlayer().getUniqueId(), event.getPlayer().getName());
    }
}
