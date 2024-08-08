package fr.kingfight.kingManageAPI.listeners;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.events.ingested.PlayerJoinE;
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
        PlayerJoinE.dispatch(event.getPlayer().getUniqueId(), event.getPlayer().getName());
    }
}
