<script setup lang="ts">

import { ref } from "vue";
import * as commands from "./commands";

import Download from "./components/Download.vue";
import Blender from "./components/Blender.vue";

const dirs = ref<DirEntry[] | undefined>(undefined);


function update_tag() {
    commands.list_dirs().then(result => {
        dirs.value = result.filter(item => item[0].includes("blender"));
    });
}

update_tag();

</script>

<template>
    <div class="main">
        <Download @update="update_tag"></Download>
        <template v-if="dirs">
            <Blender v-for="(dir, idx) in dirs"
                :path="dir[0]"
                @update="update_tag"
                :key="dir[0]"
                :created="dir[1]">
            </Blender>

        </template>
    </div>
</template>

<style scoped>
.main {
    box-sizing: border-box;
    position: relative;
    display: grid;
    grid-template-columns: auto;
    gap: 12px;
    align-items: center;
    justify-items: stretch;
    padding: 5% 5%;
}
</style>


