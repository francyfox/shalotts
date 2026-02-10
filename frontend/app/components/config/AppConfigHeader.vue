<script setup lang="ts">
import AppCopy from "~/components/copy/AppCopy.vue";
import { TerminalIcon } from "lucide-vue-next";
import AppShareDialog from "~/components/share/AppShareDialog.vue";

const configStore = useConfigStore();
const { seed, share } = storeToRefs(configStore);
</script>

<template>
  <div class="flex flex-wrap gap-2">
    <div class="w-full flex gap-1">
      <UiInput
          v-model="seed"
          placeholder="Seed"
          class="w-full"
      />
      <AppCopy :source="seed" />
      <AppShareDialog
          v-model="share.private"
          :share-url="share.url"
      />
      <UiButton>
        Install
        <TerminalIcon />
      </UiButton>
    </div>
  </div>
</template>

<style scoped>

</style>