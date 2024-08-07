import { createSharedComposable } from "@vueuse/core";
import { computed } from "vue";
import { useRouter } from "vue-router";
import {
  Book,
  CircleDollarSign,
  LineChart,
  List,
  Shield,
  Skull,
  TriangleAlert,
} from "lucide-vue-next";
import { Actions } from "./perms";
import type { Group } from "@/lib/menu";
import { useRoute } from "@/router/routes";

function _useMenu() {
  const { currentRoute } = useRouter();
  const menuList = computed<Group[]>(() => {
    return [
      /* {
        groupLabel: 'Contents',
        menus: [
          {
            href: '',
            label: 'Posts',
            active: currentRoute.value.fullPath.includes('/posts'),
            icon: SquarePen,
            submenus: [
              {
                href: '/posts',
                label: 'All Posts',
                active: currentRoute.value.fullPath === '/posts',
              },
              {
                href: '/posts/new',
                label: 'New Post',
                active: currentRoute.value.fullPath === '/posts/new',
              },
            ],
          },
          {
            href: '/categories',
            label: 'Categories',
            active: currentRoute.value.fullPath.includes('/categories'),
            icon: Bookmark,
            submenus: [],
          },
          {
            href: '/tags',
            label: 'Tags',
            active: currentRoute.value.fullPath.includes('/tags'),
            icon: Tag,
            submenus: [],
          },
        ],
      }, */
      {
        groupLabel: "Joueurs",
        action: Actions.SidebarCategoryPlayers,
        menus: [
          {
            route: "PlayersOnline",
            label: "En ligne",
            active: currentRoute.value.fullPath.includes(
              useRoute("PlayersOnline")
            ),
            action: Actions.SidebarPlayers,
            icon: List,
            submenus: [],
          },
        ],
      },
      {
        groupLabel: "Sanctions",
        action: Actions.SidebarCategorySanction,
        menus: [
          {
            route: "Sanctions",
            label: "Sanctions",
            active: currentRoute.value.fullPath.includes(useRoute("Sanctions")),
            action: Actions.SidebarSanctions,
            icon: List,
            submenus: [],
          },
          {
            route: "SanctionsLogs",
            label: "Logs",
            active: currentRoute.value.fullPath.includes(
              useRoute("SanctionsLogs")
            ),
            action: Actions.SidebarSanctionsLogs,
            icon: Book,
            submenus: [],
          },
        ],
      },
      {
        groupLabel: "Boutique",
        action: Actions.SidebarCategoryBoutique,
        menus: [
          {
            route: "BoutiqueLogs",
            label: "Logs",
            active: currentRoute.value.fullPath.includes(
              useRoute("BoutiqueLogs")
            ),
            action: Actions.SidebarBoutiqueLogs,
            icon: Book,
            submenus: [],
          },
          {
            route: "BoutiqueLogsPb",
            label: "Achats",
            active: currentRoute.value.fullPath.includes(
              useRoute("BoutiqueLogsPb")
            ),
            action: Actions.SidebarBoutiquePbLogs,
            icon: CircleDollarSign,
            submenus: [],
          },
          {
            route: "BoutiqueGraphs",
            label: "Graphiques",
            active: currentRoute.value.fullPath.includes(
              useRoute("BoutiqueGraphs")
            ),
            action: Actions.SidebarBoutiqueGraphs,
            icon: LineChart,
            submenus: [],
          },
        ],
      },
      {
        groupLabel: "Admin",
        action: Actions.SidebarCategoryAdmin,
        menus: [
          {
            route: "Staffs",
            label: "Staffs",
            active: currentRoute.value.fullPath.includes(useRoute("Staffs")),
            action: Actions.SidebarStaffs,
            icon: Shield,
            submenus: [],
          },
          {
            route: "LogsStaffs",
            label: "Logs",
            active: currentRoute.value.fullPath.includes(
              useRoute("LogsStaffs")
            ),
            action: Actions.SidebarLogsStaffs,
            icon: Book,
            submenus: [],
          },
          {
            route: "ServiceAccess",
            label: "Service Access",
            active: currentRoute.value.fullPath.includes(
              useRoute("ServiceAccess")
            ),
            action: Actions.SidebarServiceAccess,
            icon: Skull,
            submenus: [],
          },
          {
            route: "Errors",
            label: "Erreurs",
            active: currentRoute.value.fullPath.includes(useRoute("Errors")),
            action: Actions.SidebarErrors,
            icon: TriangleAlert,
            submenus: [],
          },
        ],
      },
    ];
  });

  return {
    menuList,
  };
}

export const useMenu = createSharedComposable(_useMenu);
