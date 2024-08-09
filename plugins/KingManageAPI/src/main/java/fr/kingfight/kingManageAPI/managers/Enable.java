package fr.kingfight.kingManageAPI.managers;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.tasks.FetchEventsTask;
import fr.kingfight.kingManageAPI.utils.HttpClient;
import org.bukkit.Bukkit;
import org.bukkit.ChatColor;

public class Enable {
    public HttpClient httpClient;

    public void onEnable(KingManageAPI instance) {
        long timeAtStart = System.currentTimeMillis();
        log("&3===&b=============================================&3===");
        log("- &bName&7: " + instance.getDescription().getName());
        log("- &bVersion&7: " + instance.getDescription().getVersion());
        log("- &bAuthor&7: " + instance.getDescription().getAuthors());
        log("");
        log("- &7Using &c" + instance.getServer().getVersion() + " &7version.");
        log("");

        instance.saveDefaultConfig();

        String baseUrl = instance.getConfiguration().getString("base_url");
        String apiKey = instance.getConfiguration().getString("api_key");
        Integer timeout = instance.getConfiguration().getInt("timeout");
        httpClient = new HttpClient(instance, apiKey, baseUrl, timeout);

        new CommandsManager(instance);
        new ListenersManager(instance);

        new FetchEventsTask(instance).runTaskTimerAsynchronously(instance, 0, 20L*instance.getConfiguration().getInt("intervals.events"));

        long timeAtEnd = System.currentTimeMillis();
        long timeTakenInMS = timeAtEnd - timeAtStart;
        log("- &bEnabled. Took &a" + timeTakenInMS + " &bms.");
        log("&3===&b=============================================&3===");
    }

    private static void log(String message) {
        Bukkit.getConsoleSender().sendMessage(ChatColor.translateAlternateColorCodes('&', message));
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
