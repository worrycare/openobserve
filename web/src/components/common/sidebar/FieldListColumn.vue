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
  <div
    v-if="field.ftsKey || !field.showValues"
    class="field-container flex content-center ellipsis q-pl-lg q-pr-sm"
    :title="field.name"
    style="height: 25px"
  >
    <div class="field_label ellipsis">
      {{ field.name }}
    </div>
    <div
      class="field_overlay"
      :style="{
        background: store.state.theme === 'dark' ? '#414345' : '#d9d9d9',
      }"
    >
      <template
        v-for="fieldAction in (fieldActions as any[])"
        :key="fieldAction.name"
      >
        <template
          v-if="
            typeof fieldAction.show === 'function'
              ? fieldAction.show(field)
              : fieldAction.show
          "
        >
          <q-btn
            :icon="fieldAction.icon"
            :data-test="`log-search-index-list-filter-${field.name}-field-btn`"
            style="margin-right: 0.375rem"
            size="0.4rem"
            class="q-mr-sm"
            @click.stop="handleAction(fieldAction, field, null)"
            round
          />
        </template>
      </template>
    </div>
  </div>
  <q-expansion-item
    v-else
    dense
    switch-toggle-side
    :label="field.name"
    expand-icon-class="field-expansion-icon"
    expand-icon="
                     expand_more
                  "
    @before-show="(event: any) => openFilterCreator(event, field)"
  >
    <template v-slot:header>
      <div class="flex content-center ellipsis" :title="field.name">
        <div class="field_label ellipsis">
          {{ field.name }}
        </div>
        <div class="field_overlay">
          <template
            v-for="fieldAction in (fieldActions as any[])"
            :key="fieldAction.name"
          >
            <template
              v-if="
                typeof fieldAction.show === 'function'
                  ? fieldAction.show(field)
                  : fieldAction.show
              "
            >
              <q-icon
                :name="fieldAction.icon"
                :data-test="`log-search-index-list-filter-${field.name}-field-btn`"
                :style="{
                  marginRight: '0.375rem',
                  ...(fieldAction.rounded
                    ? {
                        border: '1px solid grey',
                        borderRadius: '50%',
                        display: 'flex',
                        justifyContent: 'center',
                        alignItems: 'center',
                        padding: '2px',
                        fontSize: '12px',
                      }
                    : {}),
                }"
                size="18px"
                class="q-mr-sm cursor-pointer"
                @click.stop="handleAction(fieldAction, field, null)"
              />
            </template>
          </template>
        </div>
      </div>
    </template>
    <q-card>
      <q-card-section class="q-pl-md q-pr-xs q-py-xs">
        <div class="filter-values-container">
          <div
            v-show="fieldValues[field.name]?.isLoading"
            class="q-pl-md q-py-xs"
            style="height: 60px"
          >
            <q-inner-loading
              size="xs"
              :showing="fieldValues[field.name]?.isLoading"
              label="Fetching values..."
              label-style="font-size: 1.1em"
            />
          </div>
          <div
            v-show="
              !fieldValues[field.name]?.values?.length &&
              !fieldValues[field.name]?.isLoading
            "
            class="q-pl-md q-py-xs text-subtitle2"
          >
            No values found
          </div>
          <div
            v-for="value in fieldValues[field.name]?.values || []"
            :key="value.key"
            class="flex"
            style="height: 25px"
          >
            <div
              class="flex row wrap justify-between"
              style="width: calc(100% - 46px); font-size: 12px"
            >
              <div
                :title="value.key"
                class="ellipsis q-pr-xs"
                style="width: calc(100% - 50px)"
              >
                {{ value.key }}
              </div>
              <div
                :title="value.count"
                class="ellipsis text-right q-pr-sm"
                style="width: 50px"
              >
                {{ value.count }}
              </div>
            </div>
            <div
              class="flex row"
              style="height: fit-content"
              :class="
                store.state.theme === 'dark' ? 'text-white' : 'text-black'
              "
            >
              <template
                v-for="fieldAction in (valueActions as any[])"
                :key="fieldAction.name"
              >
                <template
                  v-if="
                    typeof fieldAction.show === 'function'
                      ? fieldAction.show(field)
                      : fieldAction.show
                  "
                >
                  <q-icon
                    :name="fieldAction.icon"
                    :data-test="`log-search-index-list-filter-${field.name}-field-btn`"
                    :style="{
                      ...(fieldAction.rounded
                        ? {
                            border: '1px solid grey',
                            borderRadius: '50%',
                            display: 'flex',
                            justifyContent: 'center',
                            alignItems: 'center',
                            padding: '3px',
                          }
                        : {}),
                    }"
                    size="10px"
                    class="q-mr-xs cursor-pointer"
                    @click.stop="handleAction(fieldAction, field, value)"
                  />
                </template>
              </template>
            </div>
          </div>
        </div>
      </q-card-section>
    </q-card>
  </q-expansion-item>
</template>
<script lang="ts">
import { defineProps } from "vue";
const props = defineProps({
  field: {
    type: Object as any,
    required: true,
    default: () => ({}),
  },
  fieldValues: {
    type: Object,
    required: true,
    default: () => ({}),
  },
  fieldActions: {
    type: Array,
    required: true,
    default: () => [],
  },
  valueActions: {
    type: Array,
    required: true,
    default: () => [],
  },
});
</script>

<style lang="scss"></style>
