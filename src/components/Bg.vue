<template>
    <div class="bg" >

    </div>
</template>

<script setup lang="ts">

import {computed} from "vue";

const props= defineProps({
  height: {
    type: Number,
    default: 0
  }
})

const h = computed(()=> `${props.height}px`)
</script>
<style lang="scss" scoped>

.bg {
  height: v-bind(h);
  width:100%;
}
</style>
