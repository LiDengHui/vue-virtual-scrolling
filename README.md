# @dhlx/virtual-scrolling 组件

一个强大的用于 Vue 应用程序的虚拟滚动组件。

## 目录

- [安装](#安装)
- [使用方法](#使用方法)
- [属性](#属性)
- [事件](#事件)
- [贡献](#贡献)
- [许可证](#许可证)

## 安装

使用 npm 安装这个包：

```bash
npm install @dhlx/virtual-scrolling
```

## 使用方法

导入`VirtualScrolling`组件并在你的 Vue 应用程序中使用它：

```html
<template>
  <VirtualScrolling :value="_data" :auto-height="true" ref="list" :default-height="48">
    <template #item="{data, index, height}">
      <Item @ready="e => setHeight(index,e)">
        <div class="item">
          {{ index }}:{{ data.text }}
        </div>
      </Item>
    </template>
  </VirtualScrolling>
</template>

<script>
import VirtualScrolling from '@dhlx/virtual-scrolling';
import Item from './Item.vue';

export default {
  components: {
    VirtualScrolling,
    Item
  },
  data() {
    return {
      _data: [/* 你的数据数组在这里 */]
    };
  },
  methods: {
    setHeight(index, height) {
      // 处理设置给定索引处的项目高度
    }
  }
};
</script>
```

## 属性

- `value`：
    - 类型：数组。
    - 默认值：一个返回空数组的函数。用于提供要在虚拟滚动列表中显示的数据数组。
- `defaultHeight`：
    - 类型：数字。
    - 默认值：50。当`autoHeight`为 false 时使用的默认高度。
- `autoHeight`：
    - 类型：布尔值。
    - 默认值：true。表示是否自动计算项目高度。
- `maxShowNumber`：
    - 类型：数字。
    - 默认值：15。设定最大显示的项目数量。

## 事件

Item    

- `ready`：由`Item`组件发出，当它准备好时。将项目的高度作为参数传递给父组件的方法。

## 暴露的方法

VirtualScrolling

- `resetListHeight(index, { clientHeight })`：用于更新列表中特定索引处的项目高度为给定的`clientHeight`。

## 贡献

欢迎贡献！请 fork 仓库并提交 pull request。

## 许可证

这个项目使用 [MIT 许可证](LICENSE)。
