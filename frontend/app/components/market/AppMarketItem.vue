<script setup lang="ts">
import { MergeIcon, GithubIcon, EyeIcon, StarIcon } from "lucide-vue-next";
import type { MarketItem } from "~/types/config.types";

const { data } = defineProps<{ data: MarketItem }>();

const stars = computed(() =>
	new Intl.NumberFormat("en-US", {
		style: "decimal",
		useGrouping: true,
	})
		.format(data.git?.stars || 0)
		.replace(/,/g, "_"),
);
</script>

<template>
  <UiCard>
    <UiCardHeader class="p-2 text-2xl">
      ## {{ data.label }}
    </UiCardHeader>

    <UiCardDescription class="px-2 pb-2 text-base">
      {{ data.description }}
    </UiCardDescription>

    <UiCardFooter class="p-3">
      <div class="w-full flex justify-between gap-2">
        <div class="flex gap-2">
          <UiButton variant="secondary">
            <MergeIcon />
            Append
          </UiButton>

          <UiButton variant="secondary">
            <GithubIcon />
            Git
          </UiButton>

          <UiButton variant="secondary">
            <EyeIcon />
            Preview
          </UiButton>
        </div>

        <div
            v-if="data.git"
            class="flex gap-2"
        >
          <UiButton variant="outline" class="pointer-events-none">
            <StarIcon />
            {{ stars }}
          </UiButton>
        </div>
      </div>

    </UiCardFooter>
  </UiCard>
</template>

<style scoped>

</style>