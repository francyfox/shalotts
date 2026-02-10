<script setup lang="ts">
import { useRoute } from "#app";
import { faker } from "@faker-js/faker";
import AppConfigHeader from "~/components/config/AppConfigHeader.vue";
import AppMenu from "~/components/menu/AppMenu.vue";
import AppNavbar from "~/components/navbar/AppNavbar.vue";
import AppSkeletonText from "~/components/skeleton/AppSkeletonText.vue";
import AppIcon from "~/assets/141216820.png";

const configStore = useConfigStore();
const { menuItems, configRoot, configData, navbar } = storeToRefs(configStore);
const route = useRoute();

route.meta.title = configRoot.value?.metadata.label;

const rootMenu = useTemplateRef("rootMenu");
const { width } = useElementSize(rootMenu);
</script>

<template>
  <header class="py-2 bg-white/5">
    <div class="container">
      <div class="flex justify-between gap-2">
        <div class="flex gap-2">
          <NuxtLink to="/">
            <img :src="AppIcon" alt="app-icon" class="flex size-8">
          </NuxtLink>
          <h1 class="text-3xl">
            <span v-if="route.meta.title"># {{ route.meta.title }}</span>
            <AppSkeletonText
                v-else
                :width="250"
            />
          </h1>
        </div>
        
        <div class=""></div>
      </div>

    </div>
  </header>

  <section class="py-4">
    <div class="container">
      <div class="flex items-start gap-2">
        <Suspense>
          <AppMenu
              :items="menuItems" class="flex-shrink-0"
              :tags="configData?.tags"
          />
        </Suspense>

        <div class="w-full flex flex-col gap-2 overflow-hidden">
          <UiCard>
            <UiCardHeader class="p-2"
                          ref="rootMenu"
            >
              <AppConfigHeader />

              <AppNavbar
                  v-if="navbar.length > 0"
                  :data="navbar"
                  :root-width="width"
              />
            </UiCardHeader>
          </UiCard>
          <slot />
        </div>
      </div>
    </div>
  </section>

  <footer class="w-full mt-auto py-2 bg-white/5">
    <div class="container">
      <div class="flex gap-1">
        Shalotts version:
        <UiBadge size="sm" variant="secondary">
          {{ configRoot?.version }}
        </UiBadge>
        .
        Powered by <a href="https://github.com/francyfox" rel="noopener" target="_blank">
        with ❤️ by
        <UiBadge size="sm" variant="secondary">
          @helizart
        </UiBadge>
      </a>. Inspired by BatiJS
      </div>
    </div>
  </footer>
</template>

<style scoped>

</style>