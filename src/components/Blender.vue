<script setup lang="ts">
import { computed, ref } from 'vue';
import * as commands from "./../commands";

interface RegexMatch {
    version: string,
    branch: string,
    versionId: string;
}

const props = defineProps({
    path: String
});

const emit = defineEmits<{
    (e: "update"): void
}>();


const isXz = computed(() => {
    if (props.path?.endsWith(".xz")) {
        return true;
    } else {
        return false;
    }
});

function parse_path(): RegexMatch {

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

    const groups = matches.groups as unknown as RegexMatch;
    return groups;
}

const versionInfo = ref<RegexMatch>(parse_path());

function extract(tarPath: string | undefined) {
    if (tarPath) {
        commands.extract_tar_xz(tarPath).then(()=> {
            emit("update");
        })
    }
}

function remove_file(filePath: string | undefined) {
    if (filePath) {
        commands.remove_file(filePath).then(()=> {
            emit("update");
        })
    }
}

function remove_dir(dirPath: string | undefined) {
    if (dirPath) {
        commands.remove_dir(dirPath).then(()=> {
            emit("update");
        })
    }
}

</script>

<template>
    <div class="grid-item">
        <div> {{ versionInfo.version }}</div>
        <div> {{ versionInfo.versionId }}</div>
        <div> {{ versionInfo.branch }}</div>

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
</template>

<style scoped>
.grid-item {
    justify-self: start;

    width: 100%;
    border-style: solid;
    border-color: white;
    border-width: 1px;
    padding: 15px 15px;
    border-radius: 12px;

    display: flex;
    gap: 15px;
    justify-content: space-between;
    align-items: center;

    /* background-image: linear-gradient(to right, rgb(50, 9, 9), rgb(58, 255, 147)); */
}



</style>