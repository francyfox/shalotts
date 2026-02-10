<script setup lang="ts">
import AppMarketButton from "~/components/market/AppMarketButton.vue";
import AppMarketList from "~/components/market/AppMarketList.vue";

const marketStore = useMarketStore();
const { market } = storeToRefs(marketStore);

await marketStore.getMarketList();
</script>

<template>
  <UiDialog >
    <UiDialogTrigger as-child>
      <AppMarketButton />
    </UiDialogTrigger>
    <UiDialogContent class="w-full max-w-[calc(100%-2rem)] xl:max-w-[calc(1200px-2rem)]">

      <UiDialogHeader>
        <UiDialogTitle class="pb-3"># Market Store</UiDialogTitle>
        <UiDialogDescription>
          <AppMarketList :data="market" />
        </UiDialogDescription>
      </UiDialogHeader>

      <UiDialogFooter class="sm:justify-start">
        <UiDialogClose as-child>
          <UiButton type="button" variant="secondary">
            Close
          </UiButton>
        </UiDialogClose>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>
</template>

<style scoped>

</style>