const { invoke } = window.__TAURI__.tauri;

let submitButton = document.getElementById('submit-button');

submitButton.addEventListener("click", () => {
    console.log("submit");
    invoke("submit_implementation_form");
});
