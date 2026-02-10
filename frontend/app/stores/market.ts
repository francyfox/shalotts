import { defineStore } from "pinia";
import { useApi } from "~/composables/api";
import type { MarketItem } from "~/types/config.types";

export const useMarketStore = defineStore("market", () => {
	const search = ref<string>("");
	const market = ref<MarketItem[]>([]);
	const total = ref<number>(0);
	const chunkIndex = ref(0);

	async function getMarketList() {
		const response = await useApi("/store");
		const data = response.data as Ref<{ total: number; items: MarketItem[] }>;
		market.value = data.value.items;
		total.value = data.value.total;
	}

	watchDebounced(
		() => search,
		(v) => {},
		{
			debounce: 500,
			maxWait: 1000,
		},
	);

	return {
		market,
		chunkIndex,
		getMarketList,
	};
});
