<template>
  <div class="virtual-scrolling" @scroll="handleScroll" ref="container">
    <div class="bg" ></div>
    <div class="infinite-list">
      <slot name="item"
            v-for="item in list.slice(first, last + 1)"
            :data="value[item.index]"
            :index="item.index"
            :height="item.height"
            :key="item.index">
      </slot>
    </div>
  </div>
</template>
<script setup lang="ts">
import {computed, ref, useTemplateRef, watch} from "vue";
import throttle from "lodash/throttle";
import binarySearch from "../utils/binarySearch";

const props = defineProps({
  value: {
    type: Array,
    default: () => []
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
  },
});

const list = ref([]);


watch(props.value, (_newData) => {
  const _list = [];
  if (Array.isArray(props.value)) {
    props.value.forEach((item, index) => {
      const _item = {
        index,
        height: Math.floor(item.defaultHeight || props.defaultHeight),
        autoHeight: item.autoHeight || props.autoHeight
      }
      _list.push(_item)
    })
    list.value = _list;
  }
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
  const _arr = [];
  let _offsetTop = 0
  list.value.forEach(item => {
    _arr.push(_offsetTop);
    _offsetTop += item.height;
  })
  containerHeight.value = _offsetTop;
  offsetTopArr.value = _arr;
}, 10);

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
  /**x
   * 下标-1 以确保 完全覆盖页面
   */
  return binarySearch(offsetTopArr.value, scrollTop.value)
});


/**
 * 计算视窗结束index
 */
let last = computed(() => {
  if (props.autoHeight) return Math.min(first.value + props.maxShowNumber, props.value.length);
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

defineExpose({
  resetListHeight
});
const offsetTop = computed(() => {
  const a = offsetTopArr.value[first.value]
  return `${a}px`
})


const h = computed(()=> `${containerHeight.value}px`)
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

.infinite-list {
  position: absolute;
  left: 0;
  right: 0;
  top: v-bind(offsetTop)
}

.bg {
  position: absolute;
  top:0;
  bottom:0;
  height: v-bind(h);
  width:100%;
}
</style>
