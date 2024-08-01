package fr.kingfight.kingManageAPI.managers;

import fr.kingfight.kingManageAPI.KingManageAPI;
import org.bukkit.Bukkit;
import org.bukkit.ChatColor;

public class Disable {
    public static void onDisable(KingManageAPI instance) {
        log("&4===&c=============================================&4===");
        log("&c" + instance.getDescription().getName() + "&7- &cDisabled");
        log("");
        log("- &cVersion&7: " + instance.getDescription().getVersion());
        log("- &cAuthor&7: " + instance.getDescription().getAuthors());
        log("");
        log("&4===&c=============================================&4===");
    }

    private static void log(String message) {
        Bukkit.getConsoleSender().sendMessage(ChatColor.translateAlternateColorCodes('&', message));
    }
}
