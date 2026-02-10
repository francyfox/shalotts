<script setup lang="ts">
import AppConfigItem from "~/components/config/AppConfigItem.vue";

const route = useRoute();
const configStore = useConfigStore();
const { menuItems } = storeToRefs(configStore);

const { error } = await configStore.getEcosystemData(
	(route.params.id as string) || "",
);

definePageMeta({
	middleware: "ecosystem",
});
</script>

<template>
  <AppConfigItem
      v-for="i in menuItems"
      :key="i.id"
      :data="i"
  />
</template>

<style scoped>

</style>