<!-- Copyright 2023 Zinc Labs Inc.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
-->

<template>
  <div :style="{ height: 'calc(100vh - 57px)', overflow: 'hidden' }">
    <div class="q-ma-sm flex">
      <q-select
        v-model="activeProject"
        :options="projects"
        label="Select Project"
        style="width: 250px"
        no-case
        padding="sm"
        dense
        stack-label
        outlined
        filled
      />
      <q-btn
        color="primary"
        label="Create Project"
        class="q-ml-sm q-px-sm"
        style="margin-left: auto; height: fit-content"
        no-caps
      />
    </div>
    <route-tabs
      dataTest="project-menu-tabs"
      :tabs="tabs"
      :activeTab="activeTab"
      direction="horizontal"
      style="width: fit-content"
    />
    <q-separator />
    <router-view v-slot="{ Component }">
      <template v-if="$route.meta.keepAlive">
        <keep-alive>
          <component
            :is="Component"
            :isRumEnabled="isRumEnabled"
            :isSessionReplayEnabled="isSessionReplayEnabled"
          />
        </keep-alive>
      </template>
      <template v-else>
        <component
          :is="Component"
          :isRumEnabled="isRumEnabled"
          :isSessionReplayEnabled="isSessionReplayEnabled"
        />
      </template>
    </router-view>
  </div>
</template>

<script setup lang="ts">
import AppTabs from "@/components/common/AppTabs.vue";
import { nextTick, onActivated, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import RouteTabs from "@/components/RouteTabs.vue";
import { useStore } from "vuex";

const router = useRouter();
const activeProject = ref<string>("default");
const projects = ref<string[]>(["default", "project1", "project2"]);

const { t } = useI18n();

const store = useStore();

const activeTab = ref<string>("dashboards");
const tabs = [
  {
    dataTest: "projects-dashboards",
    name: "dashboards",
    to: {
      name: "dashboards",
      query: {
        org_identifier: store.state.selectedOrganization.identifier,
      },
    },
    label: t("dashboard.header"),
    class: "tab_content",
  },
  {
    dataTest: "projects-alerts",
    name: "alerts",
    to: {
      name: "alerts",
      query: {
        org_identifier: store.state.selectedOrganization.identifier,
      },
    },
    label: t("alerts.header"),
    class: "tab_content",
  },
  {
    dataTest: "projects-alerts",
    name: "functions",
    to: {
      name: "functions",
      query: {
        org_identifier: store.state.selectedOrganization.identifier,
      },
    },
    label: t("function.header"),
    class: "tab_content",
  },
];

const isRumEnabled = ref<boolean>(false);
const isSessionReplayEnabled = ref<boolean>(false);

onMounted(async () => {
  console.log("mounted");
});

onActivated(async () => {});

const changeTab = async (tab: string) => {
  await nextTick();
  await nextTick();

  console.log("tab", tab);

  if (tab === "dashboards") {
    router.replace({
      name: "dashboards",
    });
    return;
  }

  if (tab === "functions") {
    router.replace({
      name: "functionList",
    });
    return;
  }

  if (tab === "alerts") {
    router.replace({
      name: "alerts",
    });
    return;
  }
};
</script>

<style scoped lang="scss">
.rum-tabs {
  border-bottom: 1px solid #e0e0e0;
  .rum-tab {
    border-bottom: 2px solid transparent;
    width: 140px;
  }
  .active {
    border-bottom: 2px solid $primary;
  }
}

.enable-rum {
  max-width: 1024px;
}
</style>
