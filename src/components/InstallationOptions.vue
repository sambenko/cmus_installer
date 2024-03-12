<template>
    <div class="installation-options-container">
      <div class="installation-options-row">
        <label for="version-select">Select Version:</label>
        <SelectVersion @version-selected="emitVersionSelected" />
      </div>
      <div class="installation-options-row">
        <label for="path-selector">Installation Path:</label>
        <PathSelector @path-selected="emitPathSelected" />
      </div>
    </div>
  </template>
  
  <script>
  import SelectVersion from './SelectVersion.vue';
  import PathSelector from './PathSelector.vue';
  
  export default {
    name: 'InstallationOptions',
    components: {
      SelectVersion,
      PathSelector,
    },
    methods: {
      emitVersionSelected(value) {
        this.$emit('version-selected', value);
      },
      emitPathSelected(value) {
        this.$emit('path-selected', value);
      },
    },
  };
  </script>
  
  <style scoped>
  .installation-options-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .installation-options-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  </style>