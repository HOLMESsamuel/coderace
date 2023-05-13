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
    // Hide the start button and show the loader
    startButton.style.display = 'none';
    loader.classList.add('show');

    logElement.innerHTML = "";

    // Invoke the race function in Rust
    invoke('race', {window: appWindow}).then(response => {
        // Hide the loader, show the result and display the start button again
        loader.classList.remove('show');
        startButton.style.display = 'block';
        resultContainer.innerHTML = response;
    });
});

const listener = await listen('LOG', (event) => {
    logMessage(event.payload);
})
