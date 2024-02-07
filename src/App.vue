<script setup lang="ts">

import { listen } from "@tauri-apps/api/event";
import * as commands from "./commands";
import { computed, ref } from "vue";

import Blender from "./components/Blender.vue";

const downloadTotal = ref<number>(0);
const progress = ref<number>(0);
const lastProgress = ref<number>(0);
const dirs = ref<string[] | undefined>(undefined);
const status = ref<string>("");

const total = computed(() => {
    if (downloadTotal.value > 0) {
        return (downloadTotal.value / (1024 * 1024)).toFixed(1);
    }
});

const progressTotal = computed(() => {
    if (progress.value > 0) {
        return (progress.value / (1024 * 1024)).toFixed(1);
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
    return (ratePerSec / 1024).toLocaleString(undefined, { maximumFractionDigits: 0 });
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
            console.log(url);

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
                update_tag();
            });

        }
    });
}

function update_tag() {
    commands.list_dirs().then(result => {
        dirs.value = result.filter(item => item.includes("blender"));
    });
}

update_tag();

</script>

<template>
    <div class="main">
        <button @click="fetch"> Test </button>

        <template v-if="downloadTotal > 0">
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

        <template v-if="dirs">
            <Blender v-for="(dir, idx) in dirs" :path="dir" @update="update_tag" :key="dir"></Blender>

        </template>
    </div>
</template>

<style scoped>
.main {
    display: grid;
    grid-template-columns: auto;
    gap: 12px;
    align-items: center;
    justify-items: center;
    padding: 5% 10%;
}

.progress-container {
    height: 25px;
    display: flex;
    align-items: center;
    gap: 15px;
    flex-grow: 0;
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
    height: 25px;
    position: relative;
    top: 0;
    left: 0;
    background-color: greenyellow;
    z-index: 100;
}
</style>


