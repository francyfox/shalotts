import { useBioEncoder } from "#build/imports";
import { defineStore } from "pinia";
import { useApi } from "~/composables/api";
// import { getDefaultSelected } from "~/lib/utils";
import type { ConfigRoot, ConfigData } from "~/types/config.types";
export const useConfigStore = defineStore("config", () => {
	const { getSeed, initEncoder } = useBioEncoder();

	const configRoot = ref<ConfigRoot>();
	const configData = ref<ConfigData>();
	const seed = ref<string>("");
	const search = ref<string>("");
	const activeTab = ref<string>("");
	const selected = ref<string[]>([]);
	const share = shallowRef({
		private: false,
		url: "",
	});

	const navbar = computed(() => {
		if (!configRoot.value) return [];
		if (configRoot.value.ecosystems.length === 0) return [];
		return configRoot.value.ecosystems;
	});

	const menuItems = computed(() => {
		if (!configRoot.value || !configData.value) return [];
		return Object.entries(configData.value.groups).map(([_, value]) => {
			return {
				...value,
				registry: configData.value?.registry.filter(
					(i) => i.group === value.id,
				),
			};
		});
	});

	async function pushPackage(packageName: string) {
		selected.value.push(packageName);
	}

	async function removePackage(packageName: string) {
		selected.value.splice(selected.value.indexOf(packageName), 1);
	}
	async function getConfigRoot() {
		const response = await useApi("/config-data");
		const data = response.data as Ref<ConfigRoot>;

		if (data.value) {
			// initEncoder(data.value);
			configRoot.value = data.value;
			seed.value = getSeed(selected.value);
			// selected.value = getDefaultSelected(data.value.registry);
		}

		return response;
	}

	async function getEcosystemData(id?: string) {
		const response = await useApi(`/api/ecosystem/${id}`);

		const data = response.data as Ref<ConfigData>;

		if (data.value) {
			configData.value = data.value;
		}

		return response;
	}

	return {
		navbar,
		share,
		seed,
		selected,
		activeTab,
		search,
		configRoot,
		configData,
		menuItems,
		getConfigRoot,
		getEcosystemData,
		pushPackage,
		removePackage,
	};
});
