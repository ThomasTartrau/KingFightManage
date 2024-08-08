import type { LucideIcon } from "lucide-vue-next";
import type { Actions } from "@/utils/perms";

export interface Submenu {
  route: string;
  label: string;
  active?: boolean;
}

export interface Menu {
  route: string;
  params?: string;
  label: string;
  active: boolean;
  action?: Actions;
  icon: LucideIcon;
  submenus: Submenu[];
}

export interface Group {
  groupLabel: string;
  action?: Actions;
  menus: Menu[];
}

export interface CollapseMenuButtonProps {
  icon: LucideIcon;
  label: string;
  active: boolean;
  submenus: Submenu[];
  // isOpen: boolean | undefined
}
