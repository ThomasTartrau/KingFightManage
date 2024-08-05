package fr.kingfight.kingManageAPI.managers;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.listeners.PlayerJoin;
import fr.kingfight.kingManageAPI.listeners.PlayerQuit;

public class ListenersManager {
    private final KingManageAPI instance;

    public ListenersManager(KingManageAPI K){
        instance = K;
        loadListeners();
    }

    public void loadListeners(){
        instance.getServer().getPluginManager().registerEvents(new PlayerJoin(), instance);
        instance.getServer().getPluginManager().registerEvents(new PlayerQuit(), instance);
    }
}
