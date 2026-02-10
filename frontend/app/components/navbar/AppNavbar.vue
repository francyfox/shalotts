<script setup lang="ts">
import { useTemplateRefsList } from "@vueuse/core";
import { EllipsisIcon } from "lucide-vue-next";

interface MenuItem {
	id: string;
	label: string;
}

const { data, rootWidth } = defineProps<{
	data: MenuItem[];
	rootWidth: number;
}>();

const widths = ref<number[]>([]);
const widthsSum = ref<number>(0);
const left = ref<number>(data.length);
const itemRefs = useTemplateRefsList<HTMLElement>();

const observer = useResizeObserver(itemRefs, (entries) => {
	widths.value = entries.map((entry) => {
		widthsSum.value += entry.borderBoxSize[0]?.inlineSize || 0;

		return entry.borderBoxSize[0]?.inlineSize || 0;
	});
});

const navbarData = shallowRef<{ items: MenuItem[]; more: MenuItem[] }>({
	items: data,
	more: [],
});

const _rootWidth = computed(() => rootWidth - 120);

function activateMoreTransition() {
	for (let i = 0, leftWidth = 0; i < widths.value.length; i += 1) {
		left.value = i;
		leftWidth += widths.value[i] as number;

		if (leftWidth > _rootWidth.value) break;
	}

	navbarData.value = {
		items: data.slice(0, left.value + 1),
		more: data.slice(left.value + 1, data.length),
	};
}

function routePath(id: string): string {
	if (!id || id === data[0]?.id) return "/";
	return `/ecosystem/${id}`;
}

watchDebounced(
	_rootWidth,
	(v) => {
		activateMoreTransition();
	},
	{ debounce: 100, maxWait: 500 },
);

onMounted(() => {
	setTimeout(() => {
		activateMoreTransition();
		observer.stop();
	});
});
</script>

<template>
  <UiButtonGroup>
    <UiButton

        variant="outline"
        :ref="itemRefs.set"
    >
      <NuxtLink
          v-for="i in navbarData.items"
          :key="i.id"
          :to="routePath(i.id)"
      >
        {{ i.label }}
      </NuxtLink>

    </UiButton>



    <UiDropdownMenu v-if="navbarData.more.length > 0">
      <UiDropdownMenuTrigger as-child>
        <UiButton variant="outline">
          <EllipsisIcon />
        </UiButton>
      </UiDropdownMenuTrigger>

      <UiDropdownMenuContent>
        <UiDropdownMenuGroup>
          <div class="max-h-[200px] overflow-y-auto">
            <NuxtLink
                v-for="i in navbarData.more"
                :key="i.id"
                :to="routePath(i.id)"
            >
              <UiDropdownMenuItem

              >
                {{ i.label }}
              </UiDropdownMenuItem>
            </NuxtLink>
          </div>
        </UiDropdownMenuGroup>
      </UiDropdownMenuContent>
    </UiDropdownMenu>
  </UiButtonGroup>
</template>

<style scoped>

</style>