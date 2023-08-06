#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}
#[allow(unused_assignments)]
#[allow(non_snake_case)]
#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0isize);
    let a = use_state(|| 0isize);
    let mut mode: isize = 0; //0:足し算,1:引き算,2:掛け算,3:割り算,4:None
    let AppendButton1 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter * 10 + 1;
            counter.set(value);
        }
    };
    let AppendButton2 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 2;
            counter.set(value);
        }
    };
    let AppendButton3 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 3;
            counter.set(value);
        }
    };
    let AppendButton4 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 4;
            counter.set(value);
        }
    };
    let AppendButton5 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 5;
            counter.set(value);
        }
    };
    let AppendButton6 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 6;
            counter.set(value);
        }
    };
    let AppendButton7 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 7;
            counter.set(value);
        }
    };
    let AppendButton8 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 8;
            counter.set(value);
        }
    };
    let AppendButton9 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10 + 9;
            counter.set(value);
        }
    };
    let AppendButton0 = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let value = *counter.clone() * 10;
            counter.set(value);
        }
    };
    let PlusButton = {
        let counter = counter.clone();
        mode = 0;
        let a = a.clone();
        move |_: MouseEvent| {
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };

    let SubButton = {
        let counter = counter.clone();
        mode = 1;
        let a = a.clone();
        move |_: MouseEvent| {
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };

    let MulButton = {
        let counter = counter.clone();
        mode = 2;
        let a = a.clone();
        move |_: MouseEvent| {
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };
    let DivButton = {
        let counter = counter.clone();
        mode = 3;
        let a = a.clone();
        move |_: MouseEvent| {
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };
    let ExeButton = {
        let counter = counter.clone();
        let a = a.clone();
        move |_: MouseEvent| {
            if mode == 0 {
                let value = *a + *counter;
                counter.set(value);
                a.set(0);
            } else if mode == 1 {
                let value = *a - *counter;
                counter.set(value);
                a.set(0);
            } else if mode == 2 {
                let value = *a * *counter;
                counter.set(value);
                a.set(0);
            } else {
                let value = *a / *counter;
                counter.set(value);
                a.set(0);
            }
        }
    };

    let ClsButton = {
        let counter = counter.clone();
        mode = 0;
        let a = a.clone();
        move |_: MouseEvent| {
            a.set(0);
            let value = 0isize;
            counter.set(value);
        }
    };
    html! {
        <div>
            <p><b>{ *a }</b></p>
            <p><b>{ *counter }</b></p>
            <br/>
            <button onclick={AppendButton1}>{"1"}</button>
            <button onclick={AppendButton2}>{"2"}</button>
            <button onclick={AppendButton3}>{"3"}</button>
            <button onclick={PlusButton}>{"+"}</button>
            <br/>
            <button onclick={AppendButton4}>{"4"}</button>
            <button onclick={AppendButton5}>{"5"}</button>
            <button onclick={AppendButton6}>{"6"}</button>
            <button onclick={SubButton}>{"-"}</button>
            <br/>
            <button onclick={AppendButton7}>{"7"}</button>
            <button onclick={AppendButton8}>{"8"}</button>
            <button onclick={AppendButton9}>{"9"}</button>
            <button onclick={MulButton}>{"×"}</button>
            <br/>
            <button onclick={ClsButton}>{"C"}</button>
            <button onclick={AppendButton0}>{"0"}</button>
            <button onclick={ExeButton}>{"="}</button>
            <button onclick={DivButton}>{"÷"}</button>
            <br/>
        </div>
    }
}
