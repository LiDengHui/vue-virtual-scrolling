<template>
  {{ first }} {{ last }}
  <div class="virtual-scrolling" @scroll="handleScroll" ref="container">
    <Bg :height="containerHeight"></Bg>
    <Item
        v-for="item in list.slice(first, last + 1)"
        :offset-top="offsetTopArr[item.index]"
        :height="item.height"
        :auto-height="item.autoHeight"
        :text="value[item.index].text"
        :key="item.index"
        @ready="d => resetListHeight(item.index, d)"
    >
    </Item>
  </div>
</template>
<script setup lang="ts">
import Bg from "./Bg.vue";
import Item from "./Item.vue";
import {computed, ref, useTemplateRef, watch} from "vue";
import throttle from "lodash/throttle";
import binarySearch from "../utils/binarySearch";
import {ListManager, process_list} from '../../wasm-demo/pkg'

const props = defineProps({
  value: {
    type: Array,
    default: []
  },
  defaultHeight: {
    type: Number,
    default: 50
  },
  autoHeight: {
    type: Boolean,
    default: true
  },
  maxShowNumber: {
    type: Number,
    default: 15
  }
});

const list = ref([]);

const listManager = new ListManager()

watch(props.value, () => {
  const _list = [];
  props.value.forEach((item, index) => {
    const _item = {
      index,
      height: Math.floor(item.defaultHeight || props.defaultHeight),
      autoHeight: item.autoHeight || props.autoHeight
    }
    _list.push(_item)
  })
  list.value = _list;
}, {
  immediate: true,
})
/**
 * 容器总高度
 */
const containerHeight = ref(0);

/**
 * 每个子元素的偏移高度
 */
const offsetTopArr = ref([]);

const cb = throttle(() => {

  console.time("wasm")
  // let x = process_list(list.value);

  const _arr = [];
  let _offsetTop = 0
  list.value.forEach(item => {
    _arr.push(_offsetTop);
    _offsetTop += item.height;
  })
  console.timeEnd("wasm")
  containerHeight.value = _offsetTop;
  offsetTopArr.value = _arr;
}, 1);

watch(() => list.value, cb, {
  immediate: true,
  deep: true
})

const containerRef = useTemplateRef("container")
const scrollTop = ref(0);
/**
 * 计算视窗开始index
 */
let first = computed(() => {
  /**
   * 二分查找最近的数组下标
   */
  const e = binarySearch(offsetTopArr.value, scrollTop.value);
  /**
   * 下标-1 以确保 完全覆盖页面
   */
  return Math.max(e - 1, 0);
});


/**
 * 计算视窗结束index
 */
let last = computed(() => {
  if (props.autoHeight) return Math.min(first.value + props.maxShowNumber, props.value.length);

  const clientHeight = containerRef.value?.clientHeight || 500;
  if (offsetTopArr.value.length === 0) return 0;
  for (let i = first.value + 1; i < Math.min(first.value + props.maxShowNumber, offsetTopArr.value.length); i++) {
    const temp = offsetTopArr.value[i];
    const next = list.value[i].height + temp;
    if (scrollTop.value + clientHeight >= temp && scrollTop.value + clientHeight <= next) {
      return i;
    }
  }
  return offsetTopArr.value.length - 1;
});

/**
 * 计算视窗列表
 */


const handleScroll = (e) => {
  scrollTop.value = e.target.scrollTop;
}
/**
 * 重新设置高度
 * @param index
 * @param clientHeight
 */
const resetListHeight = (index, {clientHeight}) => {
  list.value[index].height = clientHeight;
}

</script>

<style lang="scss" scoped>
.virtual-scrolling {
  height: 500px;
  border: 1px solid #ccc;
  overflow-y: auto;
  position: relative;
  width: 700px;
  scroll-behavior: smooth;
}
</style>
