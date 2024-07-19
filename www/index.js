import init from './target/sandshrew.js'

setInterval(loadingAnimation, 400);

let loadingState = 1;
let loadingStrings = ["Loading", "Loading.", "Loading..", "Loading...", "Loading..."]

function loadingAnimation() {
  let loadingDiv = document.getElementById("loading");
  loadingDiv.innerHTML = loadingStrings[loadingState];
  loadingState++;
  if (loadingState >= loadingStrings.length) {
    loadingState = 0;
  }
} 

init()