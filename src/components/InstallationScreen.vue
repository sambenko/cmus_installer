<template>
  <div class="installation-screen-container">
    <div class="installation-screen-header">
      <div class="installation-screen-heading">
        <h1>C* Music Player Installer</h1>
      </div>
    </div>
    <div class="installation-screen-content">
      <InstallationOptions @version-selected="versionSelected = $event" @path-selected="pathSelected = $event" />
    </div>
    <div class="installation-screen-buttons">
      <button class="cancel-button" @click="cancelInstallation" :class="{ clicked: cancelClicked }"
        @animationend="cancelClicked = false">
        Cancel
      </button>
      <button class="install-button" :disabled="!versionSelected || !pathSelected"
        :class="{ 'install-enabled': versionSelected && pathSelected }" @click="startInstallation">
        Install
      </button>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue';
import InstallationOptions from './InstallationOptions.vue';
import { useRouter } from 'vue-router'; 

export default {
  name: 'InstallationScreen',
  components: {
    InstallationOptions,
  },
  setup(_, { emit }) {
    const versionSelected = ref('');
    const pathSelected = ref('');
    const router = useRouter();

    const startInstallation = () => {
      console.log('versionSelected:', versionSelected.value);
      console.log('pathSelected:', pathSelected.value);
      if (versionSelected.value && pathSelected.value) {
        emit('start-installation', { version: versionSelected.value, path: pathSelected.value });
        router.push({ name: 'installation-progress', params: { version: versionSelected.value, path: pathSelected.value } });
      }
    };

    const cancelInstallation = () => {
      router.push('/');
    };

    return {
      versionSelected,
      pathSelected,
      startInstallation,
      cancelInstallation,
    };
  },
};
</script>

<style scoped>
.installation-screen-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  height: 90vh;
  padding: 1rem;
  background-color: var(--background-color);
}

.installation-screen-header {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  max-width: 500px;
  margin-bottom: 2rem;
}

.installation-screen-logo-heading {
  text-align: center;
}

.installation-screen-logo-heading .logo {
  height: 6em;
  padding: 1.5em;
  border-radius: 0;
}

.installation-screen-logo-heading h1 {
  color: var(--color);
  font-size: 2rem;
}

.installation-screen-content {
  width: 100%;
  max-width: 500px;
  padding: 2rem;
  background-color: var(--content-background-color);
  border-radius: 8px;
}

.installation-screen-buttons {
  display: flex;
  justify-content: space-between;
  width: 100%;
  max-width: 500px;
  margin-top: .1rem;
}

.cancel-button,
.install-button {
  padding: 0.75rem 1.5rem;
  font-size: 1.25rem;
  color: #ffffff;
  background-color: #ff4d4d;
  border-radius: 8px;
  border: 1px solid transparent;
  transition: background-color 0.2s ease;
  cursor: pointer;
}

.install-button {
  background-color: #aaaaaa;
  cursor: not-allowed;
}

.install-button:disabled {
  background-color: #aaaaaa;
  cursor: not-allowed;
}

.install-enabled:not(:disabled) {
  background-color: #4caf50;
  cursor: pointer;
}

.install-button:not(:disabled):active {
  background-color: #3e8e41;
  transform: scale(0.95);
}

.cancel-button:active {
  background-color: #e33e3e;
  transform: scale(0.95);
}

@media (max-width: 600px) {
  .installation-screen-logo-heading h1 {
    font-size: 1.75rem;
  }
}
</style>