package fr.kingfight.kingManageAPI.utils;

import fr.kingfight.kingManageAPI.KingManageAPI;
import fr.kingfight.kingManageAPI.api.events.ingested.LogsSanctionAPI;
import fr.kingfight.kingManageAPI.api.events.ingested.SendErrorAPI;
import fr.kingfight.kingManageAPI.events.ingested.LogsSanctionE;
import fr.kingfight.kingManageAPI.events.ingested.SendErrorE;
import org.bukkit.plugin.ServicePriority;

public class Providers {
    private static final KingManageAPI instance = KingManageAPI.getInstance();

    public static void register() {
        instance.getServer().getServicesManager().register(KingManageAPI.class, instance, instance, ServicePriority.High);
        instance.getServer().getServicesManager().register(LogsSanctionAPI.class, new LogsSanctionE(), instance, ServicePriority.Normal);
        instance.getServer().getServicesManager().register(SendErrorAPI.class, new SendErrorE(), instance, ServicePriority.Normal);
    }
}
