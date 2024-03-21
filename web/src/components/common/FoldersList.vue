<template>
  <div class="text-bold q-px-md q-pt-sm">
    {{ t("dashboard.folderLabel") }}
  </div>
  <div class="dashboards-tabs">
    <q-tabs
      indicator-color="transparent"
      inline-label
      vertical
      v-model="activeFolderId"
      data-test="dashboards-folder-tabs"
    >
      <q-tab
        v-for="(tab, index) in folders"
        :key="tab.folderId"
        :name="tab.folderId"
        content-class="tab_content full-width"
        :data-test="`dashboard-folder-tab-${tab.folderId}`"
      >
        <div class="full-width row justify-between no-wrap">
          <span
            style="
              white-space: nowrap;
              overflow: hidden;
              text-overflow: ellipsis;
            "
            :title="tab.name"
            >{{ tab.name }}</span
          >
          <div>
            <q-icon
              v-if="index"
              :name="outlinedEdit"
              class="q-ml-sm"
              @click.stop="editFolder(tab.folderId)"
              style="cursor: pointer; justify-self: end"
              data-test="dashboard-edit-folder-icon"
            />
            <q-icon
              v-if="index"
              :name="outlinedDelete"
              class="q-ml-sm"
              @click.stop="showDeleteFolderDialogFn(tab.folderId)"
              style="cursor: pointer; justify-self: end"
              data-test="dashboard-delete-folder-icon"
            />
          </div>
        </div>
      </q-tab>
    </q-tabs>
    <div
      class="row justify-center full-width q-px-xs q-pb-xs"
      style="position: sticky; bottom: 0px"
    >
      <q-btn
        class="text-bold no-border full-width"
        padding="sm lg"
        color="secondary"
        no-caps
        :label="t('dashboard.newFolderBtnLabel')"
        @click.stop="addFolder"
        data-test="dashboard-new-folder-btn"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  folders: {
    default: () => [],
    type: Array,
  },
});

const { t } = useI18n();
</script>

<style scoped></style>
