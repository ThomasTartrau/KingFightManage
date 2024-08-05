package fr.kingfight.kingManageAPI.listeners;

import fr.kingfight.kingManageAPI.events.ingested.PlayerQuitE;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerQuitEvent;

public class PlayerQuit implements Listener {

    @EventHandler
    public void onPlayerQuit(PlayerQuitEvent event) {
        PlayerQuitE.dispatch(event.getPlayer().getUniqueId());
    }
}
