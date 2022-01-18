const js = import("./wasm/pkg");

js.then((w) => {
  w.greet("JavaScript");
});
