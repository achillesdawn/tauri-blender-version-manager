<script setup lang="ts">
import { computed, ref } from 'vue';
import * as commands from "./../commands";
import { listen } from '@tauri-apps/api/event';


const emit = defineEmits<{
    update: [];
}>();

const downloadTotal = ref(0);
const lastProgress = ref<number>(0);
const progress = ref(0);
const status = ref("");
const currentUrl = ref("")

const total = computed(() => {
    if (downloadTotal.value > 0) {
        return (downloadTotal.value / (1000 * 1000)).toFixed(1);
    }
});

const progressTotal = computed(() => {
    if (progress.value > 0) {
        return (progress.value / (1000 * 1000)).toFixed(1);
    }
});

const percentage = computed(() => {
    if (downloadTotal.value > 0) {
        return (progress.value / downloadTotal.value) * 100;
    } else if (downloadTotal.value == progress.value) {
        return 100;
    } else {
        return 0;
    }
});

const downloadRate = computed(() => {
    const rate = progress.value - lastProgress.value;
    const ratePerSec = rate / 2.5;
    return (ratePerSec / 1000).toLocaleString(undefined, { maximumFractionDigits: 0 });
});

function fetch() {

    downloadTotal.value = 1;
    status.value = "Initializing...";

    commands.get_download_links().then(result => {
        const matches = result.matchAll(/blender-4\.1\.\d-\w+\+\w+\.\S+?-linux.x86_64-release.tar.xz/g);

        let urls = new Set<string>();

        for (const match of matches) {
            urls.add(match[0]);
        }

        for (const url of urls) {
            currentUrl.value = url

            const contentLength = listen<number>("content_length", payload => {
                downloadTotal.value = payload.payload;
                status.value = "Downloading";
            });

            const downloadProgress = listen<number>("download_progress", payload => {
                lastProgress.value = progress.value;
                progress.value = payload.payload;
            });

            const completed = listen<void>("download_finished", () => {
                progress.value = downloadTotal.value;
                status.value = "Extracting";
                commands.extract_tar_xz(url).then(() => {
                    status.value = "Done";

                    contentLength.then(unlisten => unlisten());
                    downloadProgress.then(unlisten => unlisten());
                    completed.then(unlisten => unlisten());
                });
            });

            commands.download(url).then(() => {
                emit("update");
                setTimeout(() => {
                    downloadTotal.value = 0;
                }, 3000);
            });

        }
    });
}

</script>

<template>
    <button @click="fetch"> Get </button>
    <template v-if="downloadTotal > 0">
        <div style="font-size: small;"> {{ currentUrl }}</div>
        <div class="progress-container">
            <div> {{ percentage.toFixed(1) }}% | </div>
            <div class="progress-bar">
                <div class="progress" :style="{ width: `${percentage}%` }">

                </div>
            </div>
            <div> {{ progressTotal }} / {{ total }} MB</div>
            <div> {{ downloadRate }} kb/s </div>
            <div> {{ status }}</div>
        </div>
    </template>
</template>

<style scoped>

.progress-container {
    height: 25px;
    align-items: center;
    gap: 15px;
    display: grid;
    grid-template-columns: 8% 30% 20% 15% auto;
}

.progress-bar {
    width: 200px;
    background-color: rgb(19, 19, 19);
    border-radius: 50px;
    overflow: hidden;
    border-style: solid;
    border-width: 1px;
    border-color: white;
}

.progress {
    height: 6px;
    position: relative;
    top: 0;
    left: 0;
    background-color: greenyellow;
    z-index: 100;
}
</style>