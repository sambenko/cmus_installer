<template>
    <div class="installation-progress-container">
      <div class="progress-section">
        <div class="progress-bar-background">
          <div v-if="isInstalling" class="progress-indicator" :style="{ width: `${progress}%` }"></div>
        </div>
      </div>
      <div class="output-section">
        <p v-if="!isInstalling && logs.length === 0">Waiting for installation to start...</p>
        <p v-for="(log, index) in logs" :key="index">{{ log }}</p>
      </div>
      <button class="cancel-button" @click="cancelInstallation" :class="{ clicked: cancelClicked }"
        @animationend="cancelClicked = false">
        Cancel
      </button>
    </div>
  </template>

<script>
import { ref, onMounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export default {
    name: 'InstallationProgress',
    props: {
        version: String,
        path: String,
    },
    setup(props) {
        console.log('Received props:', props);
        const isInstalling = ref(false);
        const logs = ref([]);
        const progress = ref(0);

        onMounted(() => {

          listen("progress", (e) => {
            logs.value.push(e.payload.message);
          });

            listen("download_progress", (e) => {
              const percentage = e.payload.message;
              progress.value = parseFloat(percentage);
            });

            listen('download-info', (event) => {
                logs.value.push(event.payload.message);
            });

            listen('download-success', (event) => {
                logs.value.push(event.payload.message);
            });

            listen('prg', (event) => {
                logs.value.push(event.payload.message);
            });

            const { version, path } = props;
            if (version && path) {
            setTimeout(() => {
                startInstallation(version, path);
            }, 5000);
        }
        });

        async function startInstallation(version, path) {
            isInstalling.value = true;
            logs.value.push('Starting installation...');
            try {
                const downloadResult = await invoke('download_cmus', { version: version, targetPath: path });
                logs.value.push(downloadResult);
                const decompressResult = await invoke('decompress', { sourcePath: `${path}/cmus-${version}.zip`, targetPath: path });
                logs.value.push(decompressResult);
                const installResult = await invoke('install_cmus', { targetPath: path });
                logs.value.push(installResult);
            } catch (error) {
                console.error('Installation failed:', error);
                logs.value.push(`Installation failed: ${error}`);
            }

            isInstalling.value = false;
        }

        return { isInstalling, logs, progress };
    },
};
</script>


<style scoped>
.installation-progress-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 20px;
  background-color: #333; /* Dark background */
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  width: 80%;
  margin: 50px auto;
  color: #ccc; /* Light text color */
}

.progress-section {
  width: 100%;
}

.progress-bar-background {
  width: 100%;
  height: 20px;
  background-color: #555; /* Adjusted for dark theme */
  border-radius: 10px;
  overflow: hidden;
}

.progress-indicator {
  height: 100%;
  background-color: #4caf50;
  transition: width 0.5s ease-out;
}

.output-section {
  max-height: 200px;
  overflow-y: auto;
  background-color: #444; /* Darker background for contrast */
  padding: 10px;
  border-radius: 8px;
  border: 1px solid #666; /* Adjusted for dark theme */
}

.output-section p {
  margin: 5px 0;
  font-family: monospace;
  color: #ccc; /* Light text color for readability */
}

.cancel-button {
    padding: 0.75rem 1.5rem;
  font-size: 1.25rem;
  color: #ffffff;
  background-color: #ff4d4d;
  border-radius: 8px;
  border: 1px solid transparent;
  transition: background-color 0.2s ease;
  cursor: pointer;
  width: 30%;
}

.cancel-button:active {
  background-color: #e33e3e;
  transform: scale(0.95);
}
</style>
