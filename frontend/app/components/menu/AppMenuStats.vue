<script setup lang="ts">
const { data = { apps: 0, packages: 0 } } = defineProps<{
	data: {
		apps: number;
		packages: number;
	};
}>();
</script>

<template>
  <div class="flex flex-col gap-1">
    <ul>
      <li>
        Apps: <UiBadge variant="outline">{{ data.apps }}</UiBadge>
      </li>
      <li>
        Packages: <UiBadge variant="outline">{{ data.packages }}</UiBadge>
      </li>
    </ul>
  </div>
</template>

<style scoped>

</style>