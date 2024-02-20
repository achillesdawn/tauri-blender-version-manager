<script setup lang="ts">
import { computed, ref } from 'vue';
import * as commands from "./../commands";

interface VersionInfo {
    version: string,
    branch: string,
    versionId: string;
}

const props = defineProps({
    path: String
});

const emit = defineEmits<{
    (e: "update"): void;
}>();


const isXz = computed(() => {
    if (props.path?.endsWith(".xz")) {
        return true;
    } else {
        return false;
    }
});

function parse_path(): VersionInfo {

    const result = {
        branch: "",
        version: "",
        versionId: ""
    };

    if (!props.path) {
        return result;
    }

    const regex = /blender-(?<version>\d\.\d\.\d)-(?<branch>\w+)\+(?<versionId>.+?)-linux.+$/g;


    const matches = regex.exec(props.path);

    if (!matches || !matches.groups) {
        return result;
    }

    const groups = matches.groups as unknown as VersionInfo;
    return groups;
}

const versionInfo = ref<VersionInfo>(parse_path());

function extract(tarPath: string | undefined) {
    if (tarPath) {
        commands.extract_tar_xz(tarPath).then(() => {
            emit("update");
        });
    }
}

function remove_file(filePath: string | undefined) {
    if (filePath) {
        commands.remove_file(filePath).then(() => {
            emit("update");
        });
    }
}

function remove_dir(dirPath: string | undefined) {
    if (dirPath) {
        commands.remove_dir(dirPath).then(() => {
            emit("update");
        });
    }
}

function branchColor() {
    switch (versionInfo.value.branch) {
        case "beta":
            return { backgroundColor: "var(--beta)" };

        case "stable":
            return { backgroundColor: "var(--stable)" };
        case "alpha":
            return { backgroundColor: "var(--alpha )" };
    }
}

</script>

<template>
    <div class="grid-item">

        <div class="capsule"> {{ versionInfo.version }}</div>
        <div class="capsule" :style="branchColor()"> {{ versionInfo.branch }}</div>


        <div class="version-id"> {{ versionInfo.versionId }}</div>

        <div class="buttons">
            <button v-if="!isXz" @click="remove_dir(props.path)">
                Remove
            </button>

            <button v-if="isXz" @click="extract(props.path)">
                Extract
            </button>

            <button v-if="isXz" @click="remove_file(props.path)">
                Remove
            </button>
        </div>

    </div>
</template>

<style scoped>
.grid-item {
    position: relative;
    justify-self: start;

    width: 100%;
    border-style: solid;
    border-color: rgb(204, 199, 199);
    border-width: 1px;
    padding: 15px 15px;
    border-radius: 12px;

    display: grid;
    grid-template-columns: 1fr 1fr 3fr auto;
    gap: 15px;

    align-items: center;
    justify-items: center;

    /* background-image: linear-gradient(to right, rgb(50, 9, 9), rgb(58, 255, 147)); */
}


.buttons {
    justify-self: end;
}

.capsule {
    --beta: rgb(255, 174, 92);
    --stable: rgb(156, 234, 148);
    --alpha: rgb(234, 148, 148);

    border-style: solid;
    border-color: rgb(108, 108, 108);
    border-width: 1px;
    background-color: rgb(255, 255, 255);
    padding-left: 1rem;
    padding-right: 1rem;
    border-radius: 12px;
    color: black;
    text-align: center;
    width: fit-content;
}

.version-id {
    justify-self: start;
}
</style>