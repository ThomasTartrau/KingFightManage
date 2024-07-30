<script lang="ts" setup>
import { ref } from 'vue'
import { useColorMode } from '@vueuse/core'
import { Moon, Sun } from 'lucide-vue-next'
import {
  Tooltip,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import Button from '@/components/ui/button/Button.vue'

const colorMode = useColorMode()
const buttonRef = ref(null)

async function toggleDarkMode() {
  if (
    !buttonRef.value
    // @ts-expect-error "No comment"
    || !(document as Document).startViewTransition
    || window.matchMedia('(prefers-reduced-motion: reduce)').matches
  ) {
    colorMode.value = colorMode.value === 'dark' ? 'light' : 'dark'
    return
  }

  // @ts-expect-error "No comment"
  await (document as Document).startViewTransition(() => {
    colorMode.value = colorMode.value === 'dark' ? 'light' : 'dark'
  }).ready

  const { top, left, width, height } = (
    buttonRef.value as HTMLElement
  ).getBoundingClientRect()
  const x = left + width / 2
  const y = top + height / 2
  const right = window.innerWidth - left
  const bottom = window.innerHeight - top
  const maxRadius = Math.hypot(Math.max(left, right), Math.max(top, bottom))

  document.documentElement.animate(
    {
      clipPath: [
        `circle(0px at ${x}px ${y}px)`,
        `circle(${maxRadius}px at ${x}px ${y}px)`,
      ],
    },
    {
      duration: 500,
      easing: 'ease-in-out',
      pseudoElement: '::view-transition-new(root)',
    },
  )
}
</script>

<template>
  <TooltipProvider disable-hoverable-content>
    <Tooltip :delay-duration="100">
      <TooltipTrigger as-child>
        <Button
          ref="buttonRef"
          class="rounded-full w-8 h-8 bg-background"
          variant="outline"
          size="icon"
          @click="toggleDarkMode"
        >
          <Sun
            class="w-[1.2rem] h-[1.2rem] rotate-90 scale-0 transition-transform ease-in-out duration-500 dark:rotate-0 dark:scale-100"
          />
          <Moon
            class="absolute w-[1.2rem] h-[1.2rem] rotate-0 scale-100 transition-transform ease-in-out duration-500 dark:rotate-90 dark:scale-0"
          />
        </Button>
      </TooltipTrigger>
    </Tooltip>
  </TooltipProvider>
</template>
