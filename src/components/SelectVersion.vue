<template>
  <div class="select-version-container">
    <select id="version-select" v-model="selectedVersion" class="version-select" @change="emitVersionSelected">
      <option v-if="!hasInternet" :value="defaultVersionLabel">{{ defaultVersionLabel }}</option>
      <option v-else :value="versions[0]" selected>{{ versions[0] }} (default)</option>
      <option v-for="(version, index) in versionsToDisplay" :key="index" :value="version">
        {{ version }}
      </option>
    </select>
  </div>
</template>

<script>
import { ref, computed } from 'vue';

export default {
  name: 'SelectVersion',
  emits: ['version-selected'],
  setup(_, context) {
    const selectedVersion = ref('');
    const versions = ref([]);
    const hasInternet = ref(true);
    const defaultVersionLabel = "v2.10.0 (default)";

    async function fetchVersions() {
      const repo = 'cmus/cmus';
      try {
        const response = await fetch(`https://api.github.com/repos/${repo}/tags`);
        const data = await response.json();
        versions.value = data.map((tag) => tag.name);
        selectedVersion.value = versions.value[0];
        emitVersionSelected();
      } catch (error) {
        console.error('Failed to fetch versions:', error);
        hasInternet.value = false;
        selectedVersion.value = defaultVersionLabel;
        emitVersionSelected();
      }
    }

    function emitVersionSelected() {
      context.emit('version-selected', selectedVersion.value);
    }

    const versionsToDisplay = computed(() => {
      if (hasInternet.value) {
        return versions.value.slice(1);
      }
      return versions.value;
    });

    fetchVersions();

    return {
      selectedVersion,
      versions,
      hasInternet,
      defaultVersionLabel,
      versionsToDisplay,
      emitVersionSelected,
    };
  },
};
</script>

<style scoped>
.select-version-container {
  display: flex;
  align-items: center;
  width: 100%;
}

.version-select {
  padding: 0.5rem;
  font-size: 1rem;
  border-radius: 4px;
  border: 1px solid #555; /* Adjusted for dark theme */
  background-color: #333; /* Dark background for the select */
  color: #ccc; /* Light text color for the select */
  width: 100%;
  -webkit-appearance: none; /* Remove default styling */
  -moz-appearance: none; /* Remove default styling */
  appearance: none; /* Remove default styling */
  background: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="%23ccc" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>') no-repeat right 0.5rem center / 1.5rem 1.5rem, #333; /* Adjust dropdown arrow color and background */
}

.version-select:hover {
  border-color: #777; /* Slightly lighter border on hover */
}

.version-select:focus {
  border-color: #aaa; /* Even lighter border for focus */
  outline: none; /* Remove default focus outline */
}
</style>