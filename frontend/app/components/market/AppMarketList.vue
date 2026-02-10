<script setup lang="ts">
import { useVirtualList } from "@vueuse/core";
import { useFuse } from "@vueuse/integrations/useFuse";
import { ref } from "vue";
import AppMarketItem from "~/components/market/AppMarketItem.vue";
import type { MarketItem } from "~/types/config.types";

const emit = defineEmits<{
	onLoadMore: [];
}>();

const modelSearch = defineModel<string>({ default: "" });

const { data } = defineProps<{
	data: MarketItem[];
}>();

const containerRef = ref<HTMLElement | null>(null);

const { list, containerProps, wrapperProps } = useVirtualList(data, {
	itemHeight: 200,
	overscan: 10,
});

useInfiniteScroll(
	containerRef,
	async () => {
		emit("onLoadMore");
	},
	{ distance: 8 },
);
</script>

<template>
  <div class="market-list flex flex-col gap-2">
    <UiInput v-model="modelSearch" placeholder="Search..." />
    <Suspense>
      <div class="market-list-container relative">
        <div class="overflow-y-auto max-h-[calc(75vh-40px)] pr-3 py-3">
          <div class="flex flex-col gap-2">
            <AppMarketItem
                v-for="i in data"
                :key="i.id"
                :data="i"
            />
          </div>
        </div>
      </div>

    </Suspense>
  </div>
</template>

<style scoped lang="postcss">
.market-list-container {
  &::after, &::before {
    content: "";
    display: block;
    position: absolute;
    width: 100%;
    height: 10px;
    background: linear-gradient(to top, transparent, hsl(var(--background)));
    z-index: 1;
  }

  &::after {
    bottom: 0;
    background: linear-gradient(to bottom, transparent, hsl(var(--background)));
  }
}
</style>