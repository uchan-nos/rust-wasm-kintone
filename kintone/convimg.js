(function() {
  const wasmPath = 'https://uchan-nos.github.io/rust-wasm-kintone/pkg/rust-wasm-kintone_bg.wasm';

  kintone.events.on('app.record.detail.show', async (event) => {
    const wasmLoadPromise = wasm_bindgen(wasmPath);

    const downloadUrl = kintone.api.urlForGet(
      '/k/v1/file', {fileKey: event.record.attached_file.value[0].fileKey});
    const response = await fetch(downloadUrl, {
      headers: {'X-Requested-With': 'XMLHttpRequest'}
    });
    const fileContent = await response.arrayBuffer();

    await wasmLoadPromise;

    // call Rust function 'convimg'
    const fileConverted = wasm_bindgen.convimg(
      new Uint8Array(fileContent),
      event.record.filters.value // Array of string
    );

    // show the result as an image
    let dataHexString = '';
    for (let i = 0; i < fileConverted.byteLength; i++) {
      dataHexString += String.fromCharCode(fileConverted[i]);
    }
    const resultEl = kintone.app.record.getSpaceElement('result');
    const imageEl = document.createElement('IMG');
    imageEl.src = 'data:image/png;base64,' + window.btoa(dataHexString);
    resultEl.appendChild(imageEl);
  });
}());
