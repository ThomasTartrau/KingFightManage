package fr.kingfight.kingManageAPI.utils;

import org.bukkit.Bukkit;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;
import org.json.simple.JSONObject;

public class Helpers {
    public static Boolean hasPermission(CommandSender sender, String permission) {
        return !(sender instanceof Player) || sender.hasPermission(permission);
    }

    public static void sendAPIErrorMessage(Exception e) {
        Bukkit.getConsoleSender().sendMessage("§cKingFightAPI §f» §cAn error occurred while dispatching an event: §7" + e.getMessage());
    }
}
