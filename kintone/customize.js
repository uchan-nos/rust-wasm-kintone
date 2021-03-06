(function() {
  const wasmPath = 'https://uchan-nos.github.io/rust-wasm-kintone/pkg/rust-wasm-kintone_bg.wasm';

  kintone.events.on('app.record.detail.show', async (event) => {
    console.log('initializing wasm');
    await wasm_bindgen(wasmPath);

    const greetMsg = wasm_bindgen.greet(event.record['name']['value']);
    console.log(greetMsg);

    const greetEl = kintone.app.record.getSpaceElement('greet');
    const textEl = document.createElement('P');
    textEl.textContent = greetMsg;
    greetEl.appendChild(textEl);
  });
}());
