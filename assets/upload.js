"use strict";
const CHUNK_SIZE = 1000000; // 1MB
const form = document.querySelector('form');
if (form) {
    form.addEventListener('submit', async function (e) {
        e.preventDefault();
        setOutput('');
        const destroy = form.querySelector('input[name=destroy]');
        const password = form.querySelector('input[name=password]');
        const fileInput = form.querySelector('input[name=file]');
        const file = fileInput?.files?.[0];
        const initialFormData = new FormData();
        initialFormData.append('filename', file.name);
        initialFormData.append('mime', file.type);
        initialFormData.append('destroy', destroy.value);
        initialFormData.append('password', password.value);
        try {
            const initiateUpload = await fetch('/upload', {
                method: 'POST',
                body: initialFormData,
            });
            const fileName = await initiateUpload.text();
            chunkUpload(file, fileName);
        }
        catch (err) {
            console.error(err);
            setOutput(err.toString);
        }
    });
}
async function chunkUpload(file, fileName) {
    let index = 0;
    const totalChunks = calculateTotalChunks(file.size);
    for (let chunkStart = 0; chunkStart < file.size; chunkStart += CHUNK_SIZE) {
        const chunkEnd = chunkStart + CHUNK_SIZE;
        const chunk = file.slice(chunkStart, chunkEnd);
        const formData = new FormData();
        formData.append('chunk', chunk);
        formData.append('filename', fileName);
        formData.append('index', index.toString());
        index++;
        try {
            const res = await upload(formData, [index, totalChunks]);
            setProgress(res);
        }
        catch (error) {
            setOutput(error.message);
            console.error(error);
            break;
        }
    }
    const formData = new FormData();
    formData.append('filename', fileName);
    formData.append('final', 'true');
    finish(formData);
}
async function finish(formData) {
    try {
        const data = await fetch('/upload', {
            method: 'PUT',
            body: formData,
        });
        if (!data.ok)
            throw new Error(data.statusText);
        setOutput(await data.text());
    }
    catch (err) {
        console.error(err);
        setOutput(`Error: ${err}`);
    }
}
async function upload(formData, progress, attempt) {
    if (attempt && attempt >= 5) {
        throw new Error(`Exeeded 5 retry attempts for chunk ${progress[0]}/${progress[1]}`);
    }
    try {
        const data = await fetch('/upload', {
            method: 'PUT',
            body: formData,
        });
        if (!data.ok)
            throw new Error(`${data.status}: ${data.statusText}\nServer response: ${await data.text()}`);
    }
    catch (error) {
        setOutput(`Retrying chunk ${progress[0]}/${progress[1]}\n${error}`);
        try {
            const slept = await sleep(500);
            slept;
            const retry = upload(formData, progress, attempt ? attempt + 1 : 1);
            setOutput('');
            return retry;
        }
        catch (error) {
            throw error;
        }
    }
    return progress;
}
// ------ Helper functions below ------
// TODO: probably not the best approach, but my math skills have declined over the years :p
function calculateTotalChunks(size) {
    let totalChunks = 0;
    for (let chunkStart = 0; chunkStart < size; chunkStart += CHUNK_SIZE) {
        totalChunks++;
    }
    return totalChunks;
}
function setOutput(text) {
    const output = document.querySelector('output');
    output.innerText = text;
}
function setProgress(progress) {
    let progressbar = document.querySelector('progress');
    if (!progressbar) {
        progressbar = createNode('progress', form);
    }
    const currMax = progressbar.getAttribute('max');
    const currValue = progressbar.getAttribute('value');
    if (!currMax) {
        progressbar.setAttribute('max', progress[1].toString());
        progressbar.ariaLabel = 'progressbar';
    }
    if (!currValue || parseInt(currValue) < progress[0]) {
        progressbar.setAttribute('value', progress[0].toString());
    }
    if (progress[0] === progress[1]) {
        progressbar.remove();
        setOutput('loading ...');
    }
}
function createNode(tag, parent) {
    const element = document.createElement(tag);
    return form.appendChild(element);
}
async function sleep(duration) {
    return new Promise((resolve, _) => {
        setTimeout(() => resolve(() => { }), duration);
    });
}
