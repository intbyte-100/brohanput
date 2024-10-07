// Prevent double tap zoom in Safari
let lastTouchEnd = 0;

document.addEventListener('touchend', function (event) {
    const now = (new Date()).getTime();
    if (now - lastTouchEnd <= 300) {
        event.preventDefault();
    }
    lastTouchEnd = now;
}, false);

document.addEventListener('touchmove', keepKeyboardOpen);

async function handleInputChange(event) {
    const inputField = event.target;
    let inputText = inputField.value;
    inputField.value = '  ';
    
    if (inputText.length < 2) {
        await fetch('/backspace', { method: 'POST' });
        return;
    }

    
    await sendRequest("/submit", "input", inputText.substring(2))
}

async function handleSubmit(event) {
    event.preventDefault();
    await fetch('/enter', { method: 'POST' });
}

async function handleTab(event) {
    await fetch('/tab', { method: 'POST' });
}

let arrowInterval = null;


function handleArrowKeyPress(direction, event) {
    preventKeyboardHide(event)

    sendArrowRequest(direction);
    arrowInterval = setTimeout(() => {
        arrowInterval = setInterval(() => {
            sendArrowRequest(direction);
        }, 40); 
    }, 400);
}

function preventKeyboardHide(event) {
    event.preventDefault(); 
    const inputField = document.getElementById("inputText");
    inputField.focus();
}

function stopArrowKeyPress() {
    clearTimeout(arrowInterval);
    clearInterval(arrowInterval);
    arrowInterval = null;
}


async function sendArrowRequest(direction) {
    await sendRequest("/arrow", "arrow", direction);
}


function keepKeyboardOpen() {
    const inputField = document.getElementById("inputText");
    inputField.focus();
}


var shiftPressed = false;
async function toggleShift() {
    shiftPressed = !shiftPressed; 
    handleToggleStyle("shiftButton", shiftPressed, "Shift (OFF)", "Shift (ON)")
    sendRequest("/shift", "state", shiftPressed ? "press" : "release");
}

var controlPressed = false;
async function toggleControl() {
    controlPressed = !controlPressed; 
    handleToggleStyle("controlButton", controlPressed, "Control (OFF)", "Control (ON)")
    sendRequest("/control", "state", controlPressed ? "press" : "release");
}

function handleToggleStyle(buttonID, state, normalText, pressedText) {
    const button = document.getElementById(buttonID);
    if (state) {
        button.classList.add("pressed");
        button.textContent = pressedText;
    } else {
        button.classList.remove("pressed");
        button.textContent = normalText;
    }
}

async function sendRequest(route, valueKey, value) {
    const formData = new FormData();
    formData.append(valueKey, value);

    await fetch(route, {
        method: 'POST',
        body: formData
    });
}