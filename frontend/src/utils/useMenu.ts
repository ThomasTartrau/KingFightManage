import { createSharedComposable } from "@vueuse/core";
import { computed } from "vue";
import { useRouter } from "vue-router";
import { Book, LayoutGrid, Settings, Shield, Users } from "lucide-vue-next";
import type { Group } from "@/lib/menu";
import { useRoute } from "@/router/routes";
import { Actions } from "./perms";

function _useMenu() {
  const { currentRoute } = useRouter();
  const menuList = computed<Group[]>(() => {
    return [
      {
        groupLabel: "",
        menus: [
          {
            route: "Home",
            label: "Accueil",
            active: currentRoute.value.fullPath.includes("/dashboard"),
            icon: LayoutGrid,
            submenus: [],
          },
        ],
      },
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
        groupLabel: "Paramètres",
        menus: [
          {
            route: "Settings",
            label: "Paramètres personnels",
            active: currentRoute.value.fullPath.includes(useRoute("Settings")),
            icon: Users,
            submenus: [],
          },
          {
            route: "SecuritySettings",
            label: "Paramètres de sécurité",
            active:
              currentRoute.value.fullPath.includes(useRoute("Settings")) &&
              true,
            icon: Settings,
            submenus: [],
          },
        ],
      },
      {
        groupLabel: "Admin",
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
        ],
      },
    ];
  });

  return {
    menuList,
  };
}

export const useMenu = createSharedComposable(_useMenu);
