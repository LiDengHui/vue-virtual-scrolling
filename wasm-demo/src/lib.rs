use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Array, Reflect, Object, Number};

#[wasm_bindgen]
pub struct ContainerInfo {
    container_height: usize,
    offset_top_arr: Array,  // 不再直接导出
}

#[wasm_bindgen]
impl ContainerInfo {
    // 获取 container_height 的方法
    pub fn container_height(&self) -> usize {
        self.container_height
    }

    // 获取 offset_top_arr 的方法
    pub fn offset_top_arr(&self) -> Array {
        self.offset_top_arr.clone()  // 返回克隆的 Array
    }
}

#[wasm_bindgen]
pub struct ListManager {
    list_js: Array,  // JS 数组存储高度
}

#[wasm_bindgen]
impl ListManager {
    // 初始化 list_js 数组
    #[wasm_bindgen(constructor)]
    pub fn new() -> ListManager {
        ListManager {
            list_js: Array::new(),
        }
    }

    // 向 list_js 添加元素
    pub fn init_list(&self, height: usize) {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("height"), &JsValue::from_f64(height as f64)).unwrap();
        self.list_js.push(&obj);
    }

    // 修改 list_js 中指定 index 的 height 值
    pub fn update_height(&self, index: usize, new_height: usize) {
        let item = self.list_js.get(index as u32);
        Reflect::set(&item, &JsValue::from_str("height"), &JsValue::from_f64(new_height as f64)).unwrap();
    }

    // 计算 container_height 和 offset_top_arr
    pub fn calculate_container_info(&self) -> ContainerInfo {
        let mut offset_top = 0;
        let arr = Array::new();  // 初始化偏移量数组

        for i in 0..self.list_js.length() {
            let item = self.list_js.get(i);
            let item_height: usize = Reflect::get(&item, &JsValue::from_str("height"))
                .unwrap()
                .as_f64()
                .unwrap() as usize;

            arr.push(&Number::from(offset_top as f64));  // 将 offset_top 推入 JS 数组
            offset_top += item_height;
        }

        ContainerInfo {
            container_height: offset_top,
            offset_top_arr: arr,
        }
    }

    // 获取 list_js 数组
    pub fn get_list(&self) -> Array {
        self.list_js.clone()
    }
}
