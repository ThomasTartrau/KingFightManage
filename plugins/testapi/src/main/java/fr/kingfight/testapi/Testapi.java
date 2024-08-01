package fr.kingfight.testapi;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.utils.Problem;
import org.bukkit.plugin.java.JavaPlugin;

import java.util.concurrent.CompletableFuture;

public final class Testapi extends JavaPlugin {

    @Override
    public void onEnable() {
        if (getServer().getPluginManager().getPlugin("KingManageAPI") == null) {
            getLogger().severe("KingManageAPI not found !");
            getServer().getPluginManager().disablePlugin(this);
            return;
        }
        KingManageAPI kingManageAPI = getServer().getServicesManager().load(KingManageAPI.class);
        if (kingManageAPI == null) {
            getLogger().severe("KingManageAPI not found !");
            getServer().getPluginManager().disablePlugin(this);
            return;
        } else {
            getLogger().info("KingManageAPI found !");
            CompletableFuture<Object> response = kingManageAPI.sendRequest("GET", "/staffs", (String)null);
            response.thenAccept((result) -> {
                if (result instanceof Problem) {
                    Problem problem = (Problem) result;
                    getLogger().severe("Id " + problem.getId());
                    getLogger().severe("Title " + problem.getTitle());
                    getLogger().severe("Desc " + problem.getDetail());
                } else {
                    this.getLogger().info(result.toString());
                }
            });
        }

    }

    @Override
    public void onDisable() {
        // Plugin shutdown logic
    }
}
