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
    let implementationInput = document.getElementById('impl-name');n

    // Check if any of the input fields are empty
    if (!languageInput.value || !versionInput.value || !implementationInput.value) {
        // Display an error message
        console.log('Please fill out all fields before adding the implementation.');
    } else {
        // The input fields are not empty, continue with adding the implementation
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
}

function handleFilesystemError(error) {
    console.log(error);
}

showImplButton.addEventListener('click', function () {
    showImplementations();
})

const listener = await listen('LOG', (event) => {
    logMessage(event.payload);
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

    loadFilesystem();

    // Iterate over each language in the implementations object
    for (let language in implementations) {
        let languageDiv = document.createElement("div");
        languageDiv.className = 'language';
        languageDiv.innerHTML = `<h3>${language}</h3>`;

        // Iterate over each version in this language
        for (let version in implementations[language]) {
            let versionDiv = document.createElement("div");
            versionDiv.className = 'version';
            versionDiv.innerHTML = `<h4>${version}</h4>`;

            // Iterate over each implementation in this version
            implementations[language][version].forEach(implementation => {
                let implementationDiv = document.createElement("div");
                implementationDiv.className = 'implementation';
                implementationDiv.innerHTML = `<p>${implementation}</p>`;

                versionDiv.appendChild(implementationDiv);
            });

            languageDiv.appendChild(versionDiv);
        }

        implementationsDiv.appendChild(languageDiv);
    }
}

// Call the function to initially update the implementations view
showImplementations();
