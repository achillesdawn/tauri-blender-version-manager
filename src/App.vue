<script setup lang="ts">

import { ref } from "vue";
import * as commands from "./commands";

import Download from "./components/Download.vue";
import Blender from "./components/Blender.vue";

const dirs = ref<string[] | undefined>(undefined);


function update_tag() {
    commands.list_dirs().then(result => {
        dirs.value = result.filter(item => item.includes("blender"));
    });
}

update_tag();

</script>

<template>

    <div class="main">
        <Download></Download>
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


</style>


