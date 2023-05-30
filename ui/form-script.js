const { invoke } = window.__TAURI__.tauri;

//recover data from url
let params = new URLSearchParams(window.location.search);
let languageName = params.get('language');
let versionName = params.get('version');
let implementationName = params.get('implementation');

let submitButton = document.getElementById('submit-button');
let cancelButton = document.getElementById('cancel-button');
let implementationTitle = document.getElementById('implementation-title');
implementationTitle.innerHTML = implementationName;
let errorDiv = document.getElementById("form-error");

submitButton.addEventListener("click", () => {
    let methodNameInput = document.getElementById("method-name");
    if(!methodNameInput.value) {
        errorDiv.textContent = 'Please fill out all fields before adding the implementation.';
    } else {
        invoke("submit_implementation_form");
        invoke("close_implementation_form_window");
    }

});

cancelButton.addEventListener("click", () => {
    invoke("close_implementation_form_window");
})
