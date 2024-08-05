package fr.kingfight.kingManageAPI;

import fr.kingfight.kingManageAPI.Objects.Status.Status;
import fr.kingfight.kingManageAPI.api.HttpClientAPI;
import fr.kingfight.kingManageAPI.managers.Disable;
import fr.kingfight.kingManageAPI.managers.Enable;
import fr.kingfight.kingManageAPI.utils.HttpClient;
import fr.kingfight.kingManageAPI.utils.Problem;
import org.bukkit.Bukkit;
import org.bukkit.ChatColor;
import org.bukkit.plugin.ServicePriority;
import org.bukkit.plugin.java.JavaPlugin;

import java.util.concurrent.CompletableFuture;

public final class KingManageAPI extends JavaPlugin implements HttpClientAPI {
    private static KingManageAPI instance;
    private HttpClient httpClient = null;
    private Status status;


    @Override
    public void onEnable() {
        instance = this;
        httpClient = Enable.onEnable(this);
        this.getServer().getServicesManager().register(KingManageAPI.class, this, this, ServicePriority.Normal);
        status = new Status();

        /* CompletableFuture<Object> response = httpClient.sendRequest("GET", "/staffs", null);
        response.thenAccept(result -> {
            if (result instanceof Problem) {
                Problem problem = (Problem) result;
                System.out.println("Problem: " + problem.getProblem());
            } else {
                getLogger().info(result.toString());
            }
        }); */
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
}
