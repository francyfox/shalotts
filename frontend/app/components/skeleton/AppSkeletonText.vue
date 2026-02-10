<script setup lang="ts">
const { width } = defineProps<{
	width: number;
}>();
</script>

<template>
  <span class="inline-block h-[0.8em] bg-current/20 animate-pulse rounded"
        :style="{ width: width + 'px' }">
  </span>
</template>

<style scoped>

</style>