<script setup lang="ts">
import { LayoutGrid, User } from "lucide-vue-next";
import { onMounted, onUpdated, ref } from "vue";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { UserInfo } from "@/iam";
import { emptyUserInfo, getUserInfo, logout } from "@/iam";
import CustomRouterLink from "@/components/custom-router-link.vue";

const userInfo: UserInfo = getUserInfo().value || emptyUserInfo;

const profile_picture = ref<string>("");

function _load() {
  profile_picture.value = `/profile-pictures/${userInfo.user_id}.jpeg`;
}

onMounted(() => {
  _load();
});
onUpdated(() => {
  _load();
});
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline" class="relative h-8 w-8 rounded-full">
        <Avatar class="h-8 w-8">
          <AvatarImage :src="profile_picture" alt="Avatar" />
          <AvatarFallback class="bg-transparent">
            {{ userInfo.username.charAt(0).toUpperCase() }}
            {{ userInfo.username.charAt(1).toUpperCase() }}
          </AvatarFallback>
        </Avatar>
      </Button>
    </DropdownMenuTrigger>
    <!-- <TooltipProvider>
      <Tooltip>
        <TooltipTrigger as-child>

        </TooltipTrigger>
        <TooltipContent side="bottom">
          Profile
        </TooltipContent>
      </Tooltip>
    </TooltipProvider> -->
    <DropdownMenuContent class="w-56" align="end">
      <DropdownMenuLabel class="font-normal">
        <div class="flex flex-col space-y-1">
          <p class="text-sm font-medium leading-none">
            {{ userInfo.username }}
          </p>
          <p class="text-xs leading-none text-muted-foreground">
            {{ userInfo.email }}
          </p>
        </div>
      </DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuItem class="hover:cursor-pointer" as-child>
          <CustomRouterLink route="Home">
            <LayoutGrid class="w-4 h-4 mr-3 text-muted-foreground" />
            Accueil
          </CustomRouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem class="hover:cursor-pointer" as-child>
          <CustomRouterLink route="Settings">
            <User class="w-4 h-4 mr-3 text-muted-foreground" />
            Compte
          </CustomRouterLink>
        </DropdownMenuItem>
      </DropdownMenuGroup>
      <DropdownMenuSeparator />
      <DropdownMenuItem class="hover:cursor-pointer" @click="logout">
        <User class="w-4 h-4 mr-3 text-muted-foreground" />
        Déconnexion
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
