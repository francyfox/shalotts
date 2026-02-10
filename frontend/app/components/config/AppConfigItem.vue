<script setup lang="ts">
const emit = defineEmits<{
	onRemove: [packageName: string];
	onPush: [packageName: string];
}>();

const selected = defineModel<string[]>({ default: [] });
const { data } = defineProps<{
	data: { id: string; label: string; required: boolean; registry: any };
}>();

function isActive(v: string): boolean {
	return selected.value.includes(v);
}
function handleTogglePackage(v: any, tab: any) {
	if (isActive(v.name)) {
		const hasOneRequiredSelected =
			tab?.serve?.required && selected.value.includes(v.name);

		if (!hasOneRequiredSelected) {
			emit("onRemove", v.name);
		}
	} else {
		emit("onPush", v.name);
	}
}
</script>

<template>
  <UiCard
      class="w-full"
  >
    <UiCardHeader class="p-3">
      <div class="flex gap-2 items-center">
        <div class="text-xl flex-shrink-0">
          ## {{ data.label }} <UiKbd v-if="data.required">*required</UiKbd>
        </div>
        <div class="flex w-full h-[1px] bg-secondary"></div>
      </div>
    </UiCardHeader>

    <UiCardContent class="p-3">
      <div class="flex flex-wrap gap-1">
        <UiButton
            v-for="button in data.registry"
            :key="button.id"
            :variant="isActive(button.id) ? 'default' : 'secondary'"
            @click="handleTogglePackage(button, data)"
        >
          <img
              v-if="button.icon"
              loading="lazy"
              :src="button?.icon"
              alt=""
              class="flex size-5"
          />
          {{ button.label }}
        </UiButton>
      </div>
    </UiCardContent>
  </UiCard>
</template>

<style scoped>

</style>