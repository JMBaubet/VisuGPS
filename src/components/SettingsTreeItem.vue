<template>
  <v-list-group v-if="!node.isSetting && node.children?.length > 0" :value="node.id">
    <template v-slot:activator="{ props }">
      <v-list-item v-bind="props">
        <template v-slot:title>
          <span class="text-h6 text-grey-darken-1">{{ node.title }}</span>
        </template>
      </v-list-item>
    </template>

    <SettingsTreeItem
      v-for="child in node.children"
      :key="child.id"
      :node="child"
      :settings="settings"
    />
  </v-list-group>

  <v-list-item v-else-if="node.isSetting" class="py-2">
    <StringSetting 
      v-if="settings[node.settingIndex].type === 'string'"
      v-model="settings[node.settingIndex]"
    />
    <div v-else class="text-caption text-disabled ml-12">
      Éditeur pour le type '{{ settings[node.settingIndex].type }}' non implémenté.
    </div>
  </v-list-item>
</template>

<script setup>
import StringSetting from './StringSetting.vue';

defineProps({
  node: {
    type: Object,
    required: true,
  },
  settings: {
    type: Array,
    required: true,
  }
});

defineOptions({
  name: 'SettingsTreeItem'
});
</script>
