export default defineNuxtRouteMiddleware(async (to, from) => {
	const configStore = useConfigStore();
	const { configRoot } = storeToRefs(configStore);

	if (!configRoot.value) {
		await configStore.getConfigRoot();
	}

	if (to.path === "/") {
		const homeRouteId = configRoot.value?.ecosystems[0]?.id;
		console.log(configRoot.value);

		// return navigateTo(`/ecosystem/${homeRouteId}`);
	}
	// if (to.params.id === homeRouteId || to.name) {
	// 	return navigateTo(`/ecosystem/${homeRouteId}`);
	// }
	//

	//
	// return navigateTo(`/ecosystem/${homeRouteId}`);
});
