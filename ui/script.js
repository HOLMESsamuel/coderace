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
let resultActions = document.getElementById("result-actions");
let languageDropdown = document.getElementById("impl-lang");

let implementations = {}

let pythonAvailableVersions = ["2", "3"];
let rustAvailableVersions = ["4", "5"];

function logMessage(message) {
    logElement.innerHTML += '<p>' + message + '</p>';
}

class Result {
    constructor(language, version, name, execution_time, memory_usage, image_size) {
        this.language = language;
        this.version = version;
        this.name = name;
        this.execution_time = execution_time;
        this.memory_usage = memory_usage;
        this.image_size = image_size;
    }
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
        errorDiv.textContent = 'Please fill out all fields before adding the implementation.';
    } else {
        // The input fields are not empty, continue with adding the implementation
        addImplementation(languageInput.value, versionInput.value, implementationInput.value);
    }
});

languageDropdown.addEventListener("input", () => {
    switch (languageDropdown.value) {
        case "python":
            loadVersionList(pythonAvailableVersions);
            break;
        case "rust":
            loadVersionList(rustAvailableVersions);
            break;
    }
});

function loadVersionList(list) {
    let versionDropdown = document.getElementById("impl-version");
    versionDropdown.innerHTML = "";
    for (const version of list) {
        let row = document.createElement('option');
        row.value = version;
        row.innerHTML = version;
        versionDropdown.appendChild(row);
    };
}

function addImplementation(language, version, implementation) {
    let versionInput = document.getElementById('impl-version');
    let implementationInput = document.getElementById('impl-name');
    errorDiv.textContent = '';
    invoke('open_implementation_form_window', {languageName: language, versionName: version, implementationName: implementation});
    versionInput.innerHTML = "";
    implementationInput.innerHTML = "";
}


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
    resultContainer.style.display = "block";
    resultActions.style.display = "block"
    fillResultTab(response);
}

function fillResultTab(response) {
    let results = JSON.parse(response).map(result => new Result(result.language, result.version, result.name, result.execution_time, result.memory_usage, result.image_size));
    let tableBody = document.querySelector('#results-table tbody');

    tableBody.innerHTML = '';

    results.forEach(result => {
        // Create a new table row
        let row = document.createElement('tr');

        // Create and populate cells for each property of the result
        let languageCell = document.createElement('td');
        languageCell.textContent = result.language;
        row.appendChild(languageCell);

        let versionCell = document.createElement('td');
        versionCell.textContent = result.version;
        row.appendChild(versionCell);

        let nameCell = document.createElement('td');
        nameCell.textContent = result.name;
        row.appendChild(nameCell);

        let timeCell = document.createElement('td');
        timeCell.textContent = result.execution_time;
        row.appendChild(timeCell);

        let memoryCell = document.createElement('td');
        memoryCell.textContent = result.memory_usage;
        row.appendChild(memoryCell);

        let imageSpaceCell = document.createElement('td');
        imageSpaceCell.textContent = result.image_size;
        row.appendChild(imageSpaceCell);


        tableBody.appendChild(row);
    });
}

function handleRaceError(error) {
    resultContainer.style.display = "block";
    resultContainer.innerHTML = error;
    resultActions.style.display = "block"
}

function endRace() {
    showStartButton();
    logElement.innerHTML = "race over";
}

// This function will update the implementations view
function showImplementations() {
    resultActions.style.display = "none"
    implementationsDiv.innerHTML = "";
    logElement.innerHTML = "";
    resultContainer.style.display = "none";

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
                let implementationButton = document.createElement("button");
                implementationButton.textContent = implementation;

                // Add event listener to button to stop event bubbling and call addImplementation
                implementationButton.addEventListener("click", function(e) {
                    e.stopPropagation();
                    addImplementation(language, version, implementation);
                });

                // Create delete button
                let deleteButton = document.createElement("button");
                deleteButton.textContent = "Delete";
                deleteButton.className = "deleteButton";

                // Add event listener to delete button to stop event bubbling and call deleteImplementation
                deleteButton.addEventListener("click", function(e) {
                    e.stopPropagation();
                    deleteImplementation(language, version, implementation);
                });

                implementationDiv.appendChild(implementationButton);
                implementationDiv.appendChild(deleteButton);
                versionDiv.appendChild(implementationDiv);
            });

            languageDiv.appendChild(versionDiv);
            versionCount++;
        }

        implementationsDiv.appendChild(languageDiv);
        languageCount++;
    }
}

function deleteImplementation(language, version, implementation) {
    invoke('delete_implementation_folder', {languageName: language, versionName: version, implementationName: implementation});
    loadFilesystem();
}

// Call the function to initially update the implementations view
loadFilesystem();
//set the version dropdown
loadVersionList(pythonAvailableVersions);

