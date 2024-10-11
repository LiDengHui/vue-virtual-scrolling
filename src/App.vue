<script setup>
import VirtualScrolling from "./components/VirtualScrolling.vue";
import {reactive, useTemplateRef} from "vue";
import data from "../data.json"
import Item from "./components/Item.vue";

const _data = reactive(data);

const listRef = useTemplateRef("list");

const setHeight = (index, e) => {
  listRef.value.resetListHeight(index, e)
}
</script>

<template>
  <VirtualScrolling :value="_data" :auto-height="true" ref="list" :default-item-height="48" height="500px">
    <template #item="{data, index, height}">
      <Item @ready="e=> setHeight(index,e)">
        <div class="item">
          {{ index }}:{{ data.text }}
        </div>
      </Item>
    </template>
  </VirtualScrolling>
</template>

<style scoped>
.item {
  background: #cccccc;
}
</style>
