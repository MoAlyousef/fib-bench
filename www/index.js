import init, * as wasmfib from "./wasmfib.js";

export async function myInit() {
    await init();
}

function fibonacci(n) {
    if(n < 2) {
        return 1;
    }
    else {
        return fibonacci(n-1) + fibonacci(n - 2);
    }
}

export function wasmFib(n) {
    console.time("wasmtime");
    const ret = wasmfib.fibonacci(n);
    console.timeEnd("wasmtime");
    return ret;
}

export function jsFib(n) {
    console.time("jstime");
    const ret = fibonacci(n);
    console.timeEnd("jstime");
    return ret;
}

export async function serverFib(n) {
    console.time("servertime");
    const resp = await fetch("/", {
        method: "POST",
        body: JSON.stringify(n),
    });
    const val = await resp.json();
    const ret = parseInt(val);
    console.timeEnd("servertime");
    return ret;
}