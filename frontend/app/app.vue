<script setup lang="ts">
import { Toaster, toast } from "vue-sonner";
import { toastBus, currentLoadingId } from "~/composables/api";
import "vue-sonner/style.css";
import "@fontsource-variable/nunito-sans";

useHead({
	htmlAttrs: {
		class: "dark",
	},
});

const eventQueue: any[] = [];
let isMounted = false;

const bus = toastBus;
bus.on((event) => {
	if (!isMounted) {
		eventQueue.push(event);
		return;
	}
	handleToastEvent(event);
});

function handleToastEvent(event: any) {
	if (event.type === "loading") {
		const id = toast.loading("Loading...");
		currentLoadingId.value = id;
	} else if (event.type === "dismiss") {
		if (currentLoadingId.value) {
			toast.dismiss(currentLoadingId.value);
		}
	} else if (event.type === "error") {
		toast.error(`${event.status} Error!`, {
			description: event.statusText,
			duration: 5000,
		});
	}
}

onMounted(() => {
	isMounted = true;
	eventQueue.forEach(handleToastEvent);
	eventQueue.length = 0;
});
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
    <Toaster :closeButton="true" theme="dark" />
  </NuxtLayout>
</template>
