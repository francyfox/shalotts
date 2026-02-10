<script setup lang="ts">
import AppCopy from "~/components/copy/AppCopy.vue";
import AppMarketDialog from "~/components/market/AppMarketDialog.vue";
import AppMenuStats from "~/components/menu/AppMenuStats.vue";
import { navigationMenuTriggerStyle } from "~/components/ui/navigation-menu";
import { useFuse } from "@vueuse/integrations/useFuse";
import type { Group, Registry, Tag } from "~/types/config.types";
interface MenuItem extends Group {
	registry: Registry[];
}

const model = defineModel<string>({ default: "" });
const modelTags = defineModel("tags", { default: [] });
const { items, tags } = defineProps<{ items: MenuItem[]; tags: Tag[] }>();
const { results } = useFuse(model, items, {
	fuseOptions: {
		findAllMatches: true,
		keys: ["id", "label", "registry.id", "registry.label"],
	},
});

const filtered = computed(() =>
	model.value.length === 0 ? items : results.value.map((i) => i.item),
);
</script>

<template>
  <div class="app-menu relative flex flex-col gap-2 max-w-[300px]">
    <UiCard class="sticky top-2">
      <UiCardContent class="flex flex-col gap-2 p-3">
        <AppMarketDialog />

        <AppMenuStats />
        <UiSelect
            v-model="modelTags"
            v-if="tags"
            multiple
        >
          <UiSelectTrigger class="w-full">
            <UiSelectValue placeholder="Select a tag" />
          </UiSelectTrigger>
          <UiSelectContent>
            <UiSelectGroup
                v-for="i in tags"
                :key="i.id"
            >
              <UiSelectItem :value="i.id">
                <div class="flex items-center gap-2">
                  <img
                      :src="i.icon"
                      alt=""
                      class="size-5"
                  >
                  {{ i.id }}
                </div>
              </UiSelectItem>
            </UiSelectGroup>
          </UiSelectContent>
        </UiSelect>

        <UiInput v-model="model" placeholder="Filter..." />

        <UiNavigationMenu class="flex flex-col items-start gap-2">
          <UiNavigationMenuItem v-for="i of filtered" :key="i.id">
            <UiNavigationMenuLink
                as-child
                :class="navigationMenuTriggerStyle()"
            >
              <a href="#" @click.prevent>
                {{ i.label }}
              </a>
            </UiNavigationMenuLink>
            <UiKbd v-if="i.required" class="text-md">*</UiKbd>
          </UiNavigationMenuItem>
        </UiNavigationMenu>
      </UiCardContent>
    </UiCard>
  </div>
</template>

<style scoped>

</style>