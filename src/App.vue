<script setup>
import VirtualScrolling from "./components/VirtualScrolling.vue";
import {reactive} from "vue";
import data from "../data.json"
const list = reactive(data);

</script>

<template>
  <VirtualScrolling :value="list" :auto-height="true"></VirtualScrolling>
</template>

<style scoped>

</style>
