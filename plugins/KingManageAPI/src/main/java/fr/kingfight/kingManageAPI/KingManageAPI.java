package fr.kingfight.kingManageAPI;

import org.bukkit.plugin.Plugin;
import org.bukkit.plugin.java.JavaPlugin;

public final class KingManageAPI extends JavaPlugin {
    private static KingManageAPI instance;

    @Override
    public void onEnable() {
        instance = this;
    }

    public static KingManageAPI getInstance() {
        return instance;
    }
}
