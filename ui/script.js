// Import the Tauri API
const { invoke, appWindow } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

// Get the elements
let startButton = document.getElementById('start-button');
let loader = document.getElementById('loader');
let resultContainer = document.getElementById('result');
let logElement = document.getElementById('log');
let implementationsDiv = document.getElementById("implementations");
let showImplButton = document.getElementById('show-impl');
let addImplementationButton = document.getElementById('add-implementation')
let errorDiv = document.getElementById("form-error");

let implementations = {}

function logMessage(message) {
    logElement.innerHTML += '<p>' + message + '</p>';
}

// Add a click event listener
addImplementationButton.addEventListener('click', () => {
    event.preventDefault();
    // Select all the required input fields
    let languageInput = document.getElementById('impl-lang');
    let versionInput = document.getElementById('impl-version');
    let implementationInput = document.getElementById('impl-name');

    // Check if any of the input fields are empty
    if (!languageInput.value || !versionInput.value || !implementationInput.value) {
        // Display an error message
        errorDiv.textContent = 'Please fill out all fields before adding the implementation.';
    } else {
        // The input fields are not empty, continue with adding the implementation
        errorDiv.textContent = '';
        invoke('open_implementation_form_window', {languageName: languageInput.value, versionName: versionInput.value, implementationName: implementationInput.value});
        versionInput.innerHTML = "";
        implementationInput.innerHTML = "";
    }
});


startButton.addEventListener('click', function() {

    implementationsDiv.innerHTML = "";
    hideStartButtonShowLoader();
    logElement.innerHTML = "";

// Invoke the race function in Rust
    invoke('race', { window: appWindow })
        .then(handleRaceResult)
        .catch(handleRaceError)
        .finally(endRace);
});

function loadFilesystem() {
    invoke('read_implementations_folder_for_front', { window: appWindow})
        .then(handleFilesystem)
        .catch(handleFilesystemError);
}

function handleFilesystem(response) {
    implementations = JSON.parse(response);
    showImplementations();
}

function handleFilesystemError(error) {
    console.log(error);
}

showImplButton.addEventListener('click', function () {
    loadFilesystem();
})

const listenerLog = await listen('LOG', (event) => {
    logMessage(event.payload);
})

const listenerReload = await listen('reload_implementations', () => {
    loadFilesystem();
})

function hideStartButtonShowLoader() {
    startButton.style.display = 'none';
    loader.classList.add('show');
}

function showStartButton() {
    loader.classList.remove('show');
    startButton.style.display = 'block';
}

function handleRaceResult(response) {
    resultContainer.innerHTML = response;
    showImplButton.style.display = "block"
}

function handleRaceError(error) {
    resultContainer.innerHTML = error;
    showImplButton.style.display = "block"
}

function endRace() {
    showStartButton();
    logElement.innerHTML = "race over";
}

// This function will update the implementations view
function showImplementations() {
    showImplButton.style.display = "none"
    implementationsDiv.innerHTML = "";
    logElement.innerHTML = "";
    resultContainer.innerHTML = "";

    // Iterate over each language in the implementations object
    let languageCount = 0;
    for (let language in implementations) {
        let languageDiv = document.createElement("div");
        languageDiv.className = 'language';
        languageDiv.id = `language-${languageCount}`;
        let languageTitle = document.createElement("div");
        languageTitle.className = "language-title";
        languageTitle.innerHTML = `<span class="triangle"></span><h3>${language}</h3>`; // Add a span for the triangle
        languageDiv.appendChild(languageTitle);

        // Add event listener to languageDiv
        languageTitle.addEventListener("click", function(e) {
            // Toggle the 'hidden' class on the child .version divs
            Array.from(this.parentElement.getElementsByClassName('version')).forEach(versionDiv => {
                if (versionDiv.classList.contains('hidden')) {
                    versionDiv.classList.remove('hidden');
                    this.getElementsByClassName("triangle")[0].style.transform = "rotate(90deg)"; // Rotate the triangle
                } else {
                    versionDiv.classList.add('hidden');
                    this.getElementsByClassName("triangle")[0].style.transform = "rotate(0deg)"; // Rotate the triangle
                }
            });
        });

        // Iterate over each version in this language
        let versionCount = 0;
        for (let version in implementations[language]) {
            let versionDiv = document.createElement("div");
            versionDiv.className = 'version';
            versionDiv.id = `language-${languageCount}-version-${versionCount}`;  // Add this line
            versionDiv.innerHTML = `<h4>${version}</h4>`;

            // Add event listener to versionDiv to stop event bubbling
            versionDiv.addEventListener("click", function(e) {
                e.stopPropagation();
            });

            // Iterate over each implementation in this version
            implementations[language][version].forEach(implementation => {
                let implementationDiv = document.createElement("div");
                implementationDiv.className = 'implementation';
                implementationDiv.innerHTML = `<button>${implementation}</button>`;

                // Add event listener to implementationDiv to stop event bubbling
                implementationDiv.addEventListener("click", function(e) {
                    e.stopPropagation();
                });

                versionDiv.appendChild(implementationDiv);
            });

            languageDiv.appendChild(versionDiv);
            versionCount++;
        }

        implementationsDiv.appendChild(languageDiv);
        languageCount++;
    }
}

// Call the function to initially update the implementations view
loadFilesystem();

