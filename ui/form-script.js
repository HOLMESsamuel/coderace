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

let importedFiles = [];
let writtenFiles = [];
let argumentsList = [];

let rustTypes = ['i32', 'i64', 'f32', 'f64', 'str', 'bool'];
let pythonTypes = ['string', 'int'];

class File {
    constructor(path, name, content, modifiable) {
        this.path = path;
        this.name = name;
        this.content = content;
        this.modifiable = modifiable;
    }
}

submitButton.addEventListener("click", () => {
    let errorDiv = document.getElementById("form-error");
    if(!methodNameInput.value) {
        errorDiv.textContent = 'Please fill out all fields before adding the implementation.';
    } else {
        if(importedFiles.length === 0 && writtenFiles.length === 0) {
            errorDiv.textContent = 'You need to have at least one file containing the code to benchmark.';
        } else {
            if(checkAndGetArgumentList()) {
                createAndAddConfigJson();
                invoke("submit_implementation_form", {
                    languageName: languageName,
                    versionName: versionName,
                    implementationName: implementationName,
                    importedFilesJson : JSON.stringify(importedFiles),
                    writtenFilesJson: JSON.stringify(writtenFiles)
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
            if(writtenFiles.some(f => f.name === fileName.value) || importedFiles.some(f => f.name === fileName.value)) {
                errorDiv.innerHTML = "This file name already exists, chosse another";
            } else {
                errorDiv.innerHTML = "";
                let file = new File("", fileName.value, fileContent.value, false);
                writtenFiles.push(file);
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
    addArgumentLine(null, null);
});

function addArgumentLine(type, value) {
    let argumentContainer = document.getElementById('arguments-container');
    let newArgumentRow = document.createElement('div');
    newArgumentRow.classList.add('argument-row');

    let newValueInput = document.createElement('input');
    newValueInput.type = 'text';
    newValueInput.classList.add('argument-value');
    newValueInput.placeholder = 'Argument Value';
    if(type != null) {
        newValueInput.value = value;
    }
    newArgumentRow.appendChild(newValueInput);

    let newTypeSelect = document.createElement('select');
    newTypeSelect.classList.add('argument-type');
    // Define types depending on the language
    let types = [];
    if (languageName === 'rust') {
        types = rustTypes;
    } else if(languageName === 'python') {
        types = pythonTypes;
    }
    // Add other languages and their types as necessary
    types.forEach(function(typeOption) {
        let option = document.createElement('option');
        option.value = typeOption;
        option.text = typeOption;
        newTypeSelect.appendChild(option);
    });
    if(type != null) {
        newTypeSelect.value = type;
    }
    newArgumentRow.appendChild(newTypeSelect);

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
}

function checkAndGetArgumentList() {
    let argumentContainer = document.getElementById('arguments-container');
    let argumentRows = argumentContainer.getElementsByClassName('argument-row');

    argumentsList = [];

    for(const element of argumentRows) {
        let argumentValueInput = element.getElementsByClassName('argument-value')[0];
        let argumentTypeInput = element.getElementsByClassName('argument-type')[0];

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
    let config = new File("", "config.json", JSON.stringify(jsonContent), false);
    writtenFiles.push(config);
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
        //split on both / and \
        let fileName = payload.split(/[/\\]/).pop();
        let file = new File(event.payload, fileName, "", false);
        importedFiles.push(file);
        updateFileList();
    });
}

function displayFiles(files) {
    let fileListContainer = document.getElementById("file-list");
    files.forEach((file, index) => {
        let fileLine = document.createElement('div');

        // create the delete button for this file
        let deleteButton = document.createElement('button');
        deleteButton.textContent = 'Delete';
        deleteButton.addEventListener('click', () => {
            files.splice(index, 1);
            updateFileList();
        });

        // add text and delete button to the line
        fileLine.textContent = file.name;
        fileLine.appendChild(deleteButton);

        // create modify button for the file
        if(file.modifiable) {
            let modifyButton = document.createElement('button');
            modifyButton.textContent = "Modify";
            modifyButton.addEventListener('click', () => {
                openModifyFile(file);
            });
            fileLine.appendChild(modifyButton);
        }

        fileListContainer.appendChild(fileLine);
    });
}

function openModifyFile(file) {
    let fileName = document.getElementById("file-name");
    let fileContent = document.getElementById("file-content");

    fileName.value = file.name;
    fileContent.value = file.content;
}

function updateFileList() {
    let fileListContainer = document.getElementById("file-list");
    fileListContainer.innerHTML = "";

    displayFiles(importedFiles);
    displayFiles(writtenFiles);
}

function loadData() {
    invoke('load_data', {languageName: languageName, versionName: versionName, implementationName: implementationName }).then(handleData);
}

function handleData(response) {
    let data = JSON.parse(response);
    console.log(data);
    data.files.forEach(file => {
        writtenFiles.push(file);
    });
    methodNameInput.value = data.methodName;
    data.arguments.forEach(argument => {
        addArgumentLine(argument.argument_type, argument.value);
    });
    updateFileList();
}

setupListeners();
loadData();
