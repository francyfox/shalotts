<script setup lang="ts">
import AppCopy from "~/components/copy/AppCopy.vue";
import AppShareButton from "~/components/share/AppShareButton.vue";

const { shareUrl } = defineProps<{ shareUrl: string }>();
const modelPrivate = defineModel<boolean>({ default: false });
</script>

<template>
  <UiDialog>
    <UiDialogTrigger as-child>
      <AppShareButton />
    </UiDialogTrigger>
    <UiDialogContent class="sm:max-w-md">
      <UiDialogHeader>
        <UiDialogTitle class="pb-3">Share link</UiDialogTitle>
        <UiDialogDescription>
          <div class="flex flex-col gap-2">
            <div class="flex items-center gap-2">
              <UiSwitch
                  v-model="modelPrivate"
                  id="private"
              />
              <UiLabel for="private" class="text-base leading-5">
                Private?
              </UiLabel>
            </div>
            <span v-if="modelPrivate">
                Anyone who has this link will be able to view this.
            </span>
            <span v-else>
                Available in global store and by link.
            </span>
          </div>


        </UiDialogDescription>
      </UiDialogHeader>
      <div class="flex items-center gap-2">
        <div class="grid flex-1 gap-2">
          <UiLabel for="link" class="sr-only">
            Link
          </UiLabel>
          <div class="flex gap-1">
            <UiInput
                :model-value="shareUrl"
                id="link"
                default-value="Not created yet"
                disabled
            />
            <AppCopy
                :source="shareUrl"
                :disabled="!shareUrl"
            />
          </div>
        </div>
      </div>
      <UiDialogFooter class="sm:justify-start">
        <UiButton type="button">
          Share on cloud
        </UiButton>
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