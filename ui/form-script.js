const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

//recover data from url
let params = new URLSearchParams(window.location.search);
let languageName = params.get('language');
let versionName = params.get('version');
let implementationName = params.get('implementation');

let methodNameInput = document.getElementById("method-name");

let submitButton = document.getElementById('submit-button');
let cancelButton = document.getElementById('cancel-button');
let implementationTitle = document.getElementById('implementation-title');
let addFile = document.getElementById('add-file-button');
let addFileFromComputerButton = document.getElementById('load-file-button');
implementationTitle.innerHTML = implementationName;

let importedFilePaths = [];
let importedFileNames = [];
let writtenFileContents = [];
let writtenFileNames = [];
let argumentsList = [];

submitButton.addEventListener("click", () => {
    let errorDiv = document.getElementById("form-error");
    if(!methodNameInput.value) {
        errorDiv.textContent = 'Please fill out all fields before adding the implementation.';
    } else {
        if(importedFileNames.length === 0 && writtenFileNames.length === 0) {
            errorDiv.textContent = 'You need to have at least one file containing the code to benchmark.';
        } else {
            if(checkAndGetArgumentList()) {
                createAndAddConfigJson();
                invoke("submit_implementation_form", {
                    languageName: languageName,
                    versionName: versionName,
                    implementationName: implementationName,
                    importedFileNames : importedFileNames,
                    importedFilePaths: importedFilePaths,
                    writtenFileNames: writtenFileNames,
                    writtenFileContents: writtenFileContents
                });
                emit("reload_implementations", {"message": "reload"});
                invoke("close_implementation_form_window");
            } else {
                errorDiv.innerHTML = "Please fill out all argument fields before submitting.";
            }

        }
    }

});

addFile.addEventListener("click", () => {
    let fileName = document.getElementById("file-name");
    let fileContent = document.getElementById("file-content");

    let errorDiv = document.getElementById("file-form-error");

    if(!fileName.value || !fileContent.value) {
        errorDiv.innerHTML = "You must write a file name and a content to add it.";
    } else {
        if(isValidFilename(fileName.value)) {
            if(writtenFileNames.includes(fileName.value) || importedFileNames.includes(fileName.value)) {
                errorDiv.innerHTML = "This file name already exists, chosse another";
            } else {
                errorDiv.innerHTML = "";
                writtenFileNames.push(fileName.value);
                writtenFileContents.push(fileContent.value);
                updateFileList();
                fileName.value = "";
                fileContent.value = "";
            }
        } else {
            errorDiv.innerHTML = "A valid file name with extension must be written.";
        }
    }
})

function isValidFilename(filename) {
    // Regex pattern for validating a filename with an extension.
    // It checks that the filename contains only alphanumeric characters, hyphens, underscores or spaces,
    // followed by a period and then one or more alphanumeric characters (the extension).
    const pattern = /^[a-zA-Z0-9-_]+\.[a-zA-Z0-9]+$/;
    return pattern.test(filename);
}

cancelButton.addEventListener("click", () => {
    invoke("close_implementation_form_window");
})

document.getElementById('add-argument').addEventListener('click', function() {
    let argumentContainer = document.getElementById('arguments-container');
    let newArgumentRow = document.createElement('div');
    newArgumentRow.classList.add('argument-row');

    let newValueInput = document.createElement('input');
    newValueInput.type = 'text';
    newValueInput.classList.add('argument-value');
    newValueInput.placeholder = 'Argument Value';
    newArgumentRow.appendChild(newValueInput);

    let newTypeInput = document.createElement('input');
    newTypeInput.type = 'text';
    newTypeInput.classList.add('argument-type');
    newTypeInput.placeholder = 'Argument Type';
    newArgumentRow.appendChild(newTypeInput);

    // Create delete button
    let deleteButton = document.createElement('button');
    deleteButton.innerHTML = 'Delete';
    deleteButton.classList.add('delete-argument');

    // Add event listener to delete button
    deleteButton.addEventListener('click', function() {
        argumentContainer.removeChild(newArgumentRow);
    });

    newArgumentRow.appendChild(deleteButton);

    argumentContainer.appendChild(newArgumentRow);
});

function checkAndGetArgumentList() {
    let argumentContainer = document.getElementById('arguments-container');
    let argumentRows = argumentContainer.getElementsByClassName('argument-row');

    argumentsList = [];

    for(let i = 0; i < argumentRows.length; i++) {
        let argumentValueInput = argumentRows[i].getElementsByClassName('argument-value')[0];
        let argumentTypeInput = argumentRows[i].getElementsByClassName('argument-type')[0];

        if(argumentValueInput.value && argumentTypeInput.value) {
            let argument = {
                value: argumentValueInput.value,
                argument_type: argumentTypeInput.value
            }
            argumentsList.push(argument);
        } else {
            return false;
        }
    }

    return true;

}

function createAndAddConfigJson() {
    let jsonContent = {
        "method_name": methodNameInput.value, // You will fill these fields later
        "module_name": implementationName, // You will fill these fields later
        "arguments": argumentsList
    };

    // Convert jsonContent to a string and add it to writtenFileContent
    writtenFileContents.push(JSON.stringify(jsonContent));
    writtenFileNames.push("config.json");
}


function openFilePicker() {
    // Open a file dialog
    invoke('open_file_dialog');
}

addFileFromComputerButton.addEventListener("click", function() {
    openFilePicker();
})

async function setupListeners() {
    await listen('file-selected', (event) => {
        let payload = event.payload;
        importedFilePaths.push(payload);
        //split on both / and \
        let fileName = payload.split(/[/\\]/).pop();
        importedFileNames.push(fileName);
        updateFileList();
    });
}

function updateFileList() {
    let fileListContainer = document.getElementById("file-list");
    fileListContainer.innerHTML = "";

    importedFileNames.forEach((fileName, index) => {
        let fileLine = document.createElement('div');

        // create the delete button for this file
        let deleteButton = document.createElement('button');
        deleteButton.textContent = 'Delete';
        deleteButton.addEventListener('click', () => {
            importedFilePaths.splice(index, 1);
            importedFileNames.splice(index, 1);
            updateFileList();
        });

        // add text and delete button to the line
        fileLine.textContent = fileName;
        fileLine.appendChild(deleteButton);

        fileListContainer.appendChild(fileLine);
    });

    writtenFileNames.forEach((fileName, index) => {
        let fileLine = document.createElement('div');

        // create the delete button for this file
        let deleteButton = document.createElement('button');
        deleteButton.textContent = 'Delete';
        deleteButton.addEventListener('click', () => {
            writtenFileContents.splice(index, 1);
            writtenFileNames.splice(index, 1);
            updateFileList();
        });

        // add text and delete button to the line
        fileLine.textContent = fileName;
        fileLine.appendChild(deleteButton);

        fileListContainer.appendChild(fileLine);
    });
}

function loadData() {
    invoke('load_data', {languageName: languageName, versionName: versionName, implementationName: implementationName }).then(handleData);
}

function handleData(response) {
    let data = JSON.parse(response);
    data.files.forEach(file => {
        writtenFileNames.push(file);
    });
    updateFileList();
}

setupListeners();
loadData();
