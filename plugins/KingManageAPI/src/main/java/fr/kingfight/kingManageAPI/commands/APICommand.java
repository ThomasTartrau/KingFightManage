package fr.kingfight.kingManageAPI.commands;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.Objects.Status.status;
import fr.kingfight.kingManageAPI.events.ingested.LogsSanctionE;
import fr.kingfight.kingManageAPI.events.ingested.SendErrorE;
import fr.kingfight.kingManageAPI.utils.Helpers;
import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;

import java.util.List;
import java.util.UUID;

public class APICommand implements CommandExecutor {
    private final KingManageAPI instance;

    public APICommand(KingManageAPI K){
        instance = K;
    }

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        Boolean hasPermission = Helpers.hasPermission(sender, instance.getConfiguration().getString("permissions.api"));
        if (!hasPermission) {
            sender.sendMessage(instance.getConfiguration().getString("messages.errors.permission"));
            return false;
        }
        if (args.length != 1) {
            sendHelp(sender);
            return false;
        }

        switch (args[0]) {
            case "reload":
                instance.reloadConfig();
                sender.sendMessage(instance.getConfiguration().getString("messages.success.reload"));
                break;
            case "pause":
                instance.getStatus().setStatus(status.PAUSED);
                break;
            case "resume":
                instance.getStatus().setStatus(status.ENABLED);
                break;
            default:
                sendHelp(sender);
                break;
        }

        return false;
    }

    public void sendHelp(CommandSender sender) {
        List<String> help = instance.getConfiguration().getStringList("messages.help.api");
        help.forEach(sender::sendMessage);
    }
}
