<template>
    <div class="path-selector-container">
        <input class="path-input" :value="selectedPath" placeholder="Click to choose" readonly
            @click="openPathSelectorDialog" />
        <tauri-dialog v-model="pathSelectorDialogOpen" title="Select a path" message="Select a path"
            :filters='[{ "name": "All Files", "extensions": ["*"] }]' @close="pathSelectorDialogClosed"
            @cancel="pathSelectorDialogCancelled" @open="pathSelectorDialogOpened" />
    </div>
</template>

<script>
import { ref } from 'vue';
import { open } from '@tauri-apps/api/dialog';

export default {
    name: 'PathSelector',
    emits: ['path-selected'],
    setup(props, { emit }) {
        const selectedPath = ref('');

        async function openPathSelectorDialog() {
            const selected = await open({
                directory: true,
                multiple: false,
            });

            if (Array.isArray(selected)) {
                // user selected multiple directories
            } else if (selected === null) {
                // user cancelled the selection
            } else {
                selectedPath.value = selected;
                emit('path-selected', selectedPath.value);
            }
        }

        return {
            selectedPath,
            openPathSelectorDialog,
        };
    },
};
</script>


<style scoped>

.path-selector-container {
    display: flex;
    align-items: center;
    width: 100%;
}

.path-input {
    padding: 0.5rem;
    font-size: 1rem;
    border-radius: 4px;
    border: 1px solid #555; /* Adjusted for dark theme */
    background-color: #333; /* Dark background for the input */
    color: #ccc; /* Light text color for the input */
    width: 100%;
    box-sizing: border-box;
    height: 2.5rem;
}

/* Add any additional styles for hover, focus, or active states as needed */
.path-input:hover {
    border-color: #777; /* Slightly lighter border on hover */
}

.path-input:focus {
    border-color: #aaa; /* Even lighter border for focus */
    outline: none; /* Remove default focus outline */
}

</style>