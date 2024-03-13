<template>
  <div class="installation-progress-container">
    <div class="installation-progress-header">
      <div class="installation-progress-heading">
        <h1>C* Music Player Installer</h1>
      </div>
    </div>
    <div class="progress-section">
      <div class="progress-bar-background">
        <div v-if="isInstalling" class="progress-indicator" :style="{ width: `${progress}%` }"></div>
      </div>
    </div>
    <div class="output-section">
      <p v-if="!isInstalling && logs.length === 0">Waiting for installation to start...</p>
      <p v-for="(log, index) in logs" :key="index">{{ log }}</p>
    </div>
    <div class="button-container">
      <button v-show="isInstalling" class="cancel-button" @click="cancelInstallation">Cancel</button>
      <button v-show="!isInstalling" class="finish-button" @click="finishInstallation">Finish</button>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, nextTick } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { join } from "@tauri-apps/api/path";
import { resolveResource } from "@tauri-apps/api/path";

export default {
  name: "InstallationProgress",
  props: {
    version: String,
    path: String,
  },
  setup(props) {
    console.log("Received props:", props);
    const isInstalling = ref(false);
    const logs = ref([]);
    const progress = ref(0);

    const scrollToBottom = () => {
      nextTick(() => {
        const outputSection = document.querySelector(".output-section");
        if (outputSection) {
          outputSection.scrollTop = outputSection.scrollHeight;
        }
      });
    };

    onMounted(() => {
      listen("progress", (e) => {
        logs.value.push(e.payload.message);
        scrollToBottom();
      });

      listen("download_progress", (e) => {
        const percentage = e.payload.message;
        progress.value = parseFloat(percentage);
      });

      listen("download-info", (event) => {
        logs.value.push(event.payload.message);
      });

      listen("download-success", (event) => {
        logs.value.push(event.payload.message);
      });

      listen("prg", (event) => {
        logs.value.push(event.payload.message);
      });

      const { version, path } = props;
      if (version && path) {
        startInstallation(version, path);
      }
    });

    const finishInstallation = async () => {
      console.log("Finish clicked");
      await appWindow.close();
    };

    async function startInstallation(version, path) {
      isInstalling.value = true;
      logs.value.push("Starting installation...");
      let modified_version = version.substring(1);
      try {
        if (version.includes("(default)")) {
          const embeddedVersionPath = await join(
            await resolveResource('resources'),
            "cmus-2.10.0"
          );

          const localVersionPath = await join(path, "cmus-2.10.0");

          // Copy the embedded version directory to the local environment
          await invoke("copy_dir", {
            from: embeddedVersionPath,
            to: localVersionPath,
          });

          const installResult = await invoke("install_cmus", {
            targetPath: `${path}/cmus-2.10.0`,
          });
          logs.value.push(installResult);
          scrollToBottom();
        } else {
          const downloadResult = await invoke("download_cmus", {
            version: version,
            targetPath: path,
          });
          logs.value.push(downloadResult);
          const decompressResult = await invoke("decompress", {
            sourcePath: `${path}/cmus-${version}.zip`,
            targetPath: path,
          });
          logs.value.push(decompressResult);
          const installResult = await invoke("install_cmus", {
            targetPath: `${path}/cmus-${modified_version}`,
          });
          logs.value.push(installResult);
        }
        scrollToBottom();
      } catch (error) {
        console.error("Installation failed:", error);
        logs.value.push(`Installation failed: ${error}`);
      }

      isInstalling.value = false;
    }

    return { isInstalling, logs, progress, finishInstallation };
  },
};
</script>

<style scoped>
.installation-progress-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 20px;
  background-color: #333;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  width: 93%;
  height: 93%;
  position: relative;
  overflow: hidden;
}

.installation-progress-header {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  margin-bottom: 2rem;
}

.installation-progress-heading h1 {
  color: var(--color);
  font-size: 2rem;
}

.progress-section {
  width: 100%;
}

.progress-bar-background {
  width: 100%;
  height: 30px;
  background-color: #555;
  border-radius: 10px;
  overflow: hidden;
}

.progress-indicator {
  height: 100%;
  background-color: #4caf50;
  transition: width 0.5s ease-out;
}

.output-section {
  overflow-y: auto;
  background-color: #444;
  padding: 10px;
  border-radius: 8px;
  border: 1px solid #666;
  margin-bottom: 60px;
}

.output-section p {
  margin: 5px 0;
  font-family: monospace;
  color: #ccc;
}

.button-container {
  display: flex;
  justify-content: flex-end;
  position: absolute;
  bottom: 20px;
  right: 20px;
}

.finish-button,
.cancel-button {
  padding: 0.75rem 1.5rem;
  font-size: 1.25rem;
  color: #ffffff;
  border-radius: 8px;
  border: 1px solid transparent;
  transition: background-color 0.2s ease;
  cursor: pointer;
  width: 100px;
}

.finish-button {
  background-color: #4caf50;
}

.finish-button:active,
.cancel-button:active {
  transform: scale(0.95);
}

.cancel-button {
  background-color: #ff4d4d;
}
</style>
