// Import the Tauri API
const { invoke, appWindow } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

// Get the elements
let startButton = document.getElementById('start-button');
let loader = document.getElementById('loader');
let resultContainer = document.getElementById('result');
let logElement = document.getElementById('log');

function logMessage(message) {
    logElement.innerHTML += '<p>' + message + '</p>';
}

startButton.addEventListener('click', function() {

    hideStartButton();
    logElement.innerHTML = "";

// Invoke the race function in Rust
    invoke('race', { window: appWindow })
        .then(handleRaceResult)
        .catch(handleRaceError)
        .finally(endRace);
});

const listener = await listen('LOG', (event) => {
    logMessage(event.payload);
})

function hideStartButton() {
    startButton.style.display = 'none';
    loader.classList.add('show');
}

function showStartButton() {
    loader.classList.remove('show');
    startButton.style.display = 'block';
}

function handleRaceResult(response) {
    resultContainer.innerHTML = response;
}

function handleRaceError(error) {
    resultContainer.innerHTML = error;
}

function endRace() {
    showStartButton();
    logElement.innerHTML = "race over";
}
