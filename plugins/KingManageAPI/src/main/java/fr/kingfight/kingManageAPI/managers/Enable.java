package fr.kingfight.kingManageAPI.managers;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.tasks.FetchEventsTask;
import fr.kingfight.kingManageAPI.utils.HttpClient;
import org.bukkit.Bukkit;
import org.bukkit.ChatColor;

public class Enable {

    public static HttpClient onEnable(KingManageAPI instance) {
        long timeAtStart = System.currentTimeMillis();
        log("&3===&b=============================================&3===");
        log("- &bName&7: " + instance.getDescription().getName());
        log("- &bVersion&7: " + instance.getDescription().getVersion());
        log("- &bAuthor&7: " + instance.getDescription().getAuthors());
        log("");
        log("- &7Using &c" + instance.getServer().getVersion() + " &7version.");
        log("");

        instance.saveDefaultConfig();

        String baseUrl = instance.getConfig().getString("base_url");
        String apiKey = instance.getConfig().getString("api_key");
        Integer timeout = instance.getConfig().getInt("timeout");
        HttpClient httpClient = new HttpClient(instance, apiKey, baseUrl, timeout);

        new CommandsManager(instance);

        new FetchEventsTask(instance).runTaskTimerAsynchronously(instance, 0, 20L*instance.getConfig().getInt("intervals.events"));

        long timeAtEnd = System.currentTimeMillis();
        long timeTakenInMS = timeAtEnd - timeAtStart;
        log("- &bEnabled. Took &a" + timeTakenInMS + " &bms.");
        log("&3===&b=============================================&3===");

        return httpClient;
    }

    private static void log(String message) {
        Bukkit.getConsoleSender().sendMessage(ChatColor.translateAlternateColorCodes('&', message));
    }
}
