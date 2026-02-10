<script setup lang="ts">
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from "@/components/ui/popover";
import { useClipboard } from "@vueuse/core";
import { CopyIcon, FrownIcon } from "lucide-vue-next";

const { source, disabled } = defineProps<{
	source: string;
	disabled?: boolean;
}>();
const { copy, isSupported } = useClipboard({
	source,
});
</script>

<template>
  <div class="flex">
    <UiButton
        v-if="disabled"
        v-bind="{ disabled }"
        variant="secondary"
    >
      <CopyIcon />
    </UiButton>
    <template v-else>
      <Popover v-if="isSupported">
        <PopoverTrigger>
          <UiButton
              @click="copy(source)" variant="secondary"
          >
            <CopyIcon />
          </UiButton>
        </PopoverTrigger>
        <PopoverContent>
          Text copied to clipboard
        </PopoverContent>
      </Popover>
      <Popover v-else>
        <PopoverTrigger>
          <UiButton variant="secondary" v-bind="{ disabled }">
            <FrownIcon />
          </UiButton>
        </PopoverTrigger>
        <PopoverContent>
          Clipboard API is NOT supported
        </PopoverContent>
      </Popover>
    </template>
  </div>
</template>

<style scoped>

</style>