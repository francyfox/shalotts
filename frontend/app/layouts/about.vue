<script setup lang="ts">
import { useRoute } from "#app";
import AppSkeletonText from "~/components/skeleton/AppSkeletonText.vue";
const route = useRoute();
</script>

<template>
  <header class="py-2 bg-white/5">
    <div class="container">
      <h1 class="text-3xl">
        <span v-if="route.meta.title"># {{ route.meta.title }}</span>
        <AppSkeletonText
            v-else
            :width="250"
        />
      </h1>
    </div>
  </header>

  <section class="py-2">
    <div class="container">
      <slot />
    </div>
  </section>

  <footer class="w-full mt-auto py-2 bg-white/5">
    <div class="container">
      <div class="flex">
        Powered by @helizart. Inspired by BatiJS.
      </div>
    </div>
  </footer>
</template>

<style scoped>

</style>