<template>
  <div class="item" ref="el">{{ text }}</div>
</template>
<script setup lang="ts">
import {computed, onMounted, useTemplateRef} from "vue";
import isString from "lodash/isString"

const props = defineProps({
  offsetTop: {
    type: Number,
    default: 0,
  },
  height: {
    type: Number,
    default: 0,
  },
  autoHeight: {
    type: Boolean,
    default: true
  },
  text: {
    type: String,
    default: ""
  }
})

const el = useTemplateRef("el")

const emits = defineEmits(["ready"])

const top = computed(() => `${props.offsetTop}px`)
const height = computed(() => props.autoHeight ? "auto" : `${props.height}px`)
onMounted(() => {
  emits("ready", {
    clientHeight:el.value.clientHeight
  });
})
</script>


<style lang="scss" scoped>

.item {
  position: absolute;
  top: v-bind(top);
  left: 0;
  right: 0;
  height: v-bind(height);
  background: #cccccc;
}
</style>
