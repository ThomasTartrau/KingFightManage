package fr.kingfight.kingManageAPI.managers;

import fr.kingfight.kingManageAPI.KingManageAPI;

public class ListenersManager {
    private final KingManageAPI instance;

    public ListenersManager(KingManageAPI K){
        instance = K;
        loadListeners();
    }

    public void loadListeners(){
        /*
        instance.getServer().getPluginManager().registerEvents(new PlayerInteract(), instance);
        */
    }
}
