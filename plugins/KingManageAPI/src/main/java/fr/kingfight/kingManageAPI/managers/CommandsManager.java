package fr.kingfight.kingManageAPI.managers;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.commands.APICommand;

public class CommandsManager {
    private final KingManageAPI instance;

    public CommandsManager(KingManageAPI K){
        instance = K;
        loadCommands();
    }

    public void loadCommands(){
        instance.getCommand("api").setExecutor(new APICommand(instance));
    }
}