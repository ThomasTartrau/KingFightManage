package fr.kingfight.kingManageAPI;

import fr.kingfight.kingManageAPI.Objects.Status.Status;
import fr.kingfight.kingManageAPI.api.HttpClientAPI;
import fr.kingfight.kingManageAPI.managers.Disable;
import fr.kingfight.kingManageAPI.managers.Enable;
import fr.kingfight.kingManageAPI.utils.HttpClient;
import fr.kingfight.kingManageAPI.utils.Problem;
import fr.kingfight.kingManageAPI.utils.Providers;
import org.bukkit.Bukkit;
import org.bukkit.ChatColor;
import org.bukkit.configuration.file.FileConfiguration;
import org.bukkit.plugin.ServicePriority;
import org.bukkit.plugin.java.JavaPlugin;

import java.util.concurrent.CompletableFuture;

public final class KingManageAPI extends JavaPlugin implements HttpClientAPI {
    private static KingManageAPI instance;
    private static FileConfiguration configuration;
    private HttpClient httpClient = null;
    private Status status;


    @Override
    public void onEnable() {
        instance = this;
        configuration = this.getConfig();
        Enable enable = new Enable();
        enable.onEnable(this);
        httpClient = enable.getHttpClient();

        Providers.register();

        status = new Status();
    }

    @Override
    public void onDisable() {
        Disable.onDisable(this);
        this.getServer().getServicesManager().unregister(KingManageAPI.class, this);
    }

    public static KingManageAPI getInstance() {
        return instance;
    }

    @Override
    public CompletableFuture<Object> sendRequest(String method, String urlString, String data) {
        return httpClient.sendRequest(method, urlString, data);
    }

    @Override
    public void printProblem(Problem problem) {
        Bukkit.getConsoleSender().sendMessage(ChatColor.translateAlternateColorCodes('&', "&cProblem: " + problem.getProblem()));
    }

    public Status getStatus() {
        return status;
    }

    public FileConfiguration getConfiguration() {
        return configuration;
    }
}
