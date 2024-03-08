<template>
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
        <div
          @click.stop="handleAction(fieldAction, field, null)"
          class="cursor-pointer"
          :style="{
            marginRight: '0.375rem',
            ...(fieldAction.rounded
              ? {
                  border: '1px solid #bebebe',
                  borderRadius: '50%',
                  display: 'flex',
                  justifyContent: 'center',
                  alignItems: 'center',
                  width: '20px',
                  height: '20px',
                }
              : {}),
          }"
        >
          <q-icon
            :name="fieldAction.icon"
            :data-test="`log-search-index-list-filter-${field.name}-field-btn`"
            :size="fieldAction.rounded ? '14px' : '19px'"
            class="q-mr-sm cursor-pointer"
            @click.stop="handleAction(fieldAction, field, null)"
          />
        </div>
      </template>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useStore } from "vuex";

defineProps({
  field: {
    type: Object,
    required: true,
    default: () => ({}),
  },
  fieldActions: {
    type: Array,
    required: true,
    default: () => [],
  },
});

const store = useStore();
const emit = defineEmits(["event-emitted"]);
const handleAction = (action: string, field: any, value: any) => {
  emit("event-emitted", action, { field, value });
};
</script>

<style lang="scss" scoped></style>
