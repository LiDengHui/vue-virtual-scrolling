# VirtualScrolling 组件

一个强大的用于 Vue 应用程序的虚拟滚动组件。

## 目录

- [VirtualScrolling 组件](#virtualscrolling-组件)
  - [目录](#目录)
  - [安装](#安装)
  - [使用方法](#使用方法)
  - [属性](#属性)
  - [事件](#事件)
  - [暴露的方法](#暴露的方法)
  - [贡献](#贡献)
  - [许可证](#许可证)

## 安装

使用 npm 安装这个包：

```bash
npm install @dhlx/virtual-scrolling
```

## 使用方法

以下是在你的 Vue 项目中使用`VirtualScrolling`组件的示例：

```vue
<script setup>
import VirtualScrolling from "@dhlx/virtual-scrolling";
import { reactive, useTemplateRef } from "vue";
import data from "../data.json";
import "@dhlx/virtual-scrolling/dist/style.css";
const Item = VirtualScrolling.Item;

const _data = reactive(data);

const listRef = useTemplateRef("list");

const setHeight = (index, e) => {
  if (listRef.value) {
    listRef.value.resetListHeight(index, e);
  }
};
</script>

<template>
  <VirtualScrolling :value="_data" :auto-height="true" ref="list" :default-item-height="48" height="500px">
    <template #item="{data, index, height}">
      <Item @ready="e => setHeight(index,e)">
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
```

在上述代码中，首先从`@dhlx/virtual-scrolling`引入`VirtualScrolling`组件和`Item`子组件，通过`reactive`将数据转为响应式数据，使用`useTemplateRef`获取模板引用。在模板中，使用`VirtualScrolling`组件并传入相应的属性和`ref`，通过插槽定义`Item`的结构，并在`Item`组件上监听`ready`事件，触发`setHeight`方法来处理高度设置，通过模板引用调用`resetListHeight`方法更新特定索引处的项目高度。

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
- `default-item-height`：用于设置每个项目的默认高度。

## 事件

- `ready`：由`Item`组件发出，当它准备好时。将项目的高度作为参数传递给父组件的方法。

## 暴露的方法

- `resetListHeight(index, { clientHeight })`：用于更新列表中特定索引处的项目高度为给定的`clientHeight`。

## 贡献

欢迎贡献！请 fork 仓库并提交 pull request。

## 许可证

这个项目使用 [MIT 许可证](LICENSE)。