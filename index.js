const container = document.createElement("div");
container.className = `
h-screen
flex
content-center
items-center
justify-center
`;
const loading = document.createElement("div");
loading.className = `
animate-ping
h-4
w-4
bg-blue-600
rounded-full
`;
const button = document.createElement("button");
button.className = `
text-white
bg-gradient-to-br
from-pink-500
to-orange-400
hover:bg-gradient-to-bl
focus:ring-4
focus:outline-none
focus:ring-pink-200
dark:focus:ring-pink-800
font-medium
rounded-lg
text-sm
px-5
py-2.5
text-center
mr-2
mb-2
`;
button.textContent = "play";

(async () => {
  const audioCtx = new AudioContext();
  document.body.appendChild(container);
  container.appendChild(loading);

  const wasm = await import("./pkg");
  const buffs = await Promise.all(
    [
      await import("./se/cancel.mp3"),
      await import("./se/change.mp3"),
      await import("./se/delete.mp3"),
      await import("./se/landing.mp3"),
      await import("./se/ok.mp3"),
    ].map(
      (src) =>
        new Promise((resolve, _) => {
          var xhr = new XMLHttpRequest();
          xhr.responseType = "arraybuffer";
          xhr.open("GET", src.default);
          xhr.addEventListener("load", () => {
            audioCtx.decodeAudioData(xhr.response, (audioBuffer) => {
              resolve(audioBuffer);
            });
          });
          xhr.send();
        })
    )
  );

  button.addEventListener("click", () => {
    container.remove();
    wasm.run(buffs);
  });
  loading.remove();
  container.appendChild(button);
})();
