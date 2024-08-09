package fr.kingfight.kingManageAPI.events.fetched;

import fr.kingfight.kingManageAPI.KingManageAPI;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;

import java.util.UUID;

public class SanctionsE {
    private UUID player_id;
    private String staff_name;
    private String sanction_name;
    private String sanction_type;
    private Long sanction_duration; // in seconds
    private String sanction_motif;

    public SanctionsE(UUID player_id, String staff_name, String sanction_name, String sanction_type, Long sanction_duration, String sanction_motif) {
        this.player_id = player_id;
        this.staff_name = staff_name;
        this.sanction_name = sanction_name;
        this.sanction_type = sanction_type;
        this.sanction_duration = sanction_duration;
        this.sanction_motif = sanction_motif;
    }

    public void dispatch() {
        Player player = Bukkit.getPlayer(player_id);
        if (player == null) return;

        String command = KingManageAPI.getInstance().getConfiguration().getString("commands.sanctions." + sanction_type);
        command.replace("{player}", player.getName());
        command.replace("{motif}", sanction_motif);
        command.replace("{time}", sanction_duration.toString());
        command.replace("{staff_name}", staff_name);

        Bukkit.dispatchCommand(Bukkit.getConsoleSender(), command);
    }
}
