import init, { mutate } from "/pkg/bkbi_wasm.js";

const main = async () => {
  await init();
  const x = Array.from([1, 2, 3]);
  mutate(x);
  console.log(x);
};

main();
