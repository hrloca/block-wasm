import "./bgm/Sparkling.mp3";
import "./se/cancel.mp3";
import "./se/change.mp3";
import "./se/delete.mp3";
import "./se/landing.mp3";
import "./se/ok.mp3";

(async () => {
  const wasm = await import("./pkg");
  const button = document.createElement("button");
  button.classList.add("border-black");
  button.classList.add("border-2");
  button.classList.add("rounded");
  button.classList.add("absolute");
  button.classList.add("inset-2/4");
  button.classList.add("translate-y-[-50%]");
  button.classList.add("translate-x-[-50%]");

  button.classList.add("w-32");
  button.classList.add("h-10");

  button.textContent = "play";

  button.addEventListener("click", () => {
    wasm.run();
    button.remove();
  });

  document.body.appendChild(button);
})();
