<template>
  <div ref="el">
    <slot>{{ text }}</slot>
  </div>
</template>
<script setup lang="ts">
import {computed, onMounted, useTemplateRef} from "vue";
import isString from "lodash/isString"

defineProps({
  text: {
    type: String,
    default: ""
  }
})
const el = useTemplateRef("el")
const emits = defineEmits(["ready"])
onMounted(() => {
  emits("ready", {
    clientHeight: el.value.clientHeight
  });
})
</script>
<style lang="scss" scoped>

</style>
