export default function () {
  const table = fetch('/strings/en-us.json').then((r) => r.json());
  const fill = async () => {
    const strings = await table;
    for (const node of document.querySelectorAll('[data-string]')) {
      node.textContent = strings[node.dataset.string];
    }
  };
  const fail = async (key) => {
    const strings = await table;
    const message = document.getElementById('jellium-boot-message');
    message.textContent = strings[key];
    document.getElementById('jellium-boot').dataset.state = 'failed';
  };
  return {
    onStart: fill,
    onProgress: () => {},
    onComplete: () => {},
    onSuccess: () => {},
    onFailure: () => fail('bootWasmFailed')
  };
}
