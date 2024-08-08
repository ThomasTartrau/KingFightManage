<script setup lang="ts">
import { Codemirror } from "vue-codemirror";
import { json } from "@codemirror/lang-json";
import { oneDark } from "@codemirror/theme-one-dark";
import { EditorView } from "codemirror";
import { computed, ref } from "vue";
import { push } from "notivue";
import { CopyIcon } from "lucide-vue-next";

defineOptions({
  inheritAttrs: false,
});

const props = defineProps<Props>();
interface Props {
  content?: string;
}
const content = computed(() => props.content ?? "");

const extensions = [json(), oneDark, EditorView.lineWrapping];

// Codemirror EditorView instance ref
const view = ref<EditorView | undefined>(undefined);

function handleReady(payload: Record<string, unknown>) {
  view.value = payload.view as EditorView;
}

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(content.value);
    push.success({
      title: "Copié",
      message: "Le contenu a été copié dans le presse-papiers.",
    });
  } catch {
    push.error({
      title: "Erreur",
      message: "Une erreur est survenue lors de la copie du contenu.",
    });
  }
}
</script>

<template>
  <div class="relative">
    <Codemirror
      v-model="content"
      :style="{ minHeight: '100px' }"
      :autofocus="true"
      :indent-with-tab="true"
      :tab-size="2"
      :extensions="extensions"
      :disabled="true"
      class="rounded"
      @ready="handleReady"
    />
    <button class="absolute top-0 right-0 m-2" @click="copyToClipboard">
      <CopyIcon class="w-4 h-4 mr-2" color="white" />
    </button>
  </div>
</template>
