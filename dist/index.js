import { invoke } from "@tauri-apps/api"

invoke("greet", { name: "Worudo" })
  .then(response => console.log(response))

const randMin = document.getElementById("randMin");
const randMax = document.getElementById("randMax");
const generateResult = document.getElementById("generateResult");
const generateButton = document.getElementById("generateButton");

generateButton.addEventListener("click", ev => {
  const min = parseInt(randMin.value, 10) || 0
  const max = parseInt(randMax.value, 10) || 0
  invoke("generate", { min: min, max: max }).then(result => {
    generateResult.innerText = result
  })
});