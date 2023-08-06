use yew::prelude::*;

#[allow(unused_assignments)]
#[allow(non_snake_case)]
#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0isize);
    let a = use_state(|| 0isize);
    let mode = use_state(|| 0isize);
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
        let mode = mode.clone();
        let a = a.clone();
        move |_: MouseEvent| {
            mode.set(0);
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };

    let SubButton = {
        let counter = counter.clone();
        let mode = mode.clone();
        let a = a.clone();
        move |_: MouseEvent| {
            mode.set(1);
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };

    let MulButton = {
        let counter = counter.clone();
        let mode = mode.clone();
        let a = a.clone();
        move |_: MouseEvent| {
            mode.set(2);
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };
    let DivButton = {
        let counter = counter.clone();
        let mode = mode.clone();
        let a = a.clone();
        move |_: MouseEvent| {
            mode.set(3);
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };
    let ModButton = {
        let counter = counter.clone();
        let mode = mode.clone();
        let a = a.clone();
        move |_: MouseEvent| {
            mode.set(4);
            a.set(*counter.clone());
            let value = 0isize;
            counter.set(value);
        }
    };
    let ExeButton = {
        let counter = counter.clone();
        let a = a.clone();
        let mode = mode.clone();
        move |_: MouseEvent| {
            if *mode == 0 {
                let value = *a + *counter;
                counter.set(value);
                a.set(0);
            } else if *mode == 1 {
                let value = *a - *counter;
                counter.set(value);
                a.set(0);
            } else if *mode == 2 {
                let value = *a * *counter;
                counter.set(value);
                a.set(0);
            } else if *mode == 3 {
                let value = *a / *counter;
                counter.set(value);
                a.set(0);
            } else {
                let value = *a % *counter;
                counter.set(value);
                a.set(0);
            }
            mode.set(0);
        }
    };

    let ClsButton = {
        let counter = counter.clone();
        let mode = mode.clone();
        let a = a.clone();
        move |_: MouseEvent| {
            mode.set(0);
            a.set(0);
            let value = 0isize;
            counter.set(value);
        }
    };

    let RtButton = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let mut a: isize = 1;
            let mut b: isize = *counter;
            while b - a > 1 {
                let d = (a + b) / 2;
                if d * d > *counter {
                    b = d;
                } else {
                    a = d;
                }
            }
            counter.set(a);
        }
    };
    let SquareButton = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let r = *counter * *counter;
            counter.set(r);
        }
    };
    let FactorialButton = {
        let counter = counter.clone();
        move |_: MouseEvent| {
            let s = (1..=*counter).fold(1, |f, x| f * x);
            counter.set(s);
        }
    };
    html! {
        <div>
            <br/>
            <p><b>{ *counter }</b></p>
            <button onclick={AppendButton1}>{"1"}</button>
            <button onclick={AppendButton2}>{"2"}</button>
            <button onclick={AppendButton3}>{"3"}</button>
            <button onclick={PlusButton}>{"+"}</button>
            <button onclick={RtButton}>{"√"}</button>
            <br/>
            <button onclick={AppendButton4}>{"4"}</button>
            <button onclick={AppendButton5}>{"5"}</button>
            <button onclick={AppendButton6}>{"6"}</button>
            <button onclick={SubButton}>{"-"}</button>
            <button onclick={FactorialButton}>{"x!"}</button>
            <br/>
            <button onclick={AppendButton7}>{"7"}</button>
            <button onclick={AppendButton8}>{"8"}</button>
            <button onclick={AppendButton9}>{"9"}</button>
            <button onclick={MulButton}>{"×"}</button>
            <button onclick={SquareButton}>{"x^2"}</button>
            <br/>
            <button onclick={ClsButton}>{"C"}</button>
            <button onclick={AppendButton0}>{"0"}</button>
            <button onclick={ExeButton}>{"="}</button>
            <button onclick={DivButton}>{"÷"}</button>
            <button onclick={ModButton}>{"%"}</button>
            <br/>
        </div>
    }
}
