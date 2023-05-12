// Import the Tauri API
const { invoke } = window.__TAURI__.tauri;

// Get the button and loader elements
let startButton = document.getElementById('start-button');
let loader = document.getElementById('loader');
let resultContainer = document.getElementById('result');

startButton.addEventListener('click', function() {
    // Hide the start button and show the loader
    startButton.style.display = 'none';
    loader.classList.add('show');

    // Invoke the race function in Rust
    invoke('race').then(response => {
        // Hide the loader, show the result and display the start button again
        loader.classList.remove('show');
        startButton.style.display = 'block';
        resultContainer.innerHTML = response;
    });
});
