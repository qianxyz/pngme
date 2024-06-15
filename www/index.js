import * as wasm from "wasm-pngme";

const upload = document.getElementById("upload");
const preview = document.getElementById("preview");
var imageBytes;

upload.addEventListener("change", () => {
  if (upload.files && upload.files[0]) {
    const reader = new FileReader();
    reader.onload = (e) => {
      imageBytes = new Uint8Array(e.target.result);
      // Display the uploaded image
      preview.src =
        "data:image/png;base64," + Buffer.from(imageBytes).toString("base64");
    };
    reader.readAsArrayBuffer(upload.files[0]);
  }
});

// Encode

const encodeButton = document.getElementById("encodeButton");
const encodeChunkType = document.getElementById("encodeChunkType");
const encodeMessage = document.getElementById("encodeMessage");

encodeButton.addEventListener("click", () => {
  if (!imageBytes) {
    alert("Please upload an image first");
    return;
  }

  const chunkTypeValue = encodeChunkType.value || null;
  const messageValue = encodeMessage.value;

  imageBytes = wasm.encode(imageBytes, chunkTypeValue, messageValue);
  preview.src =
    "data:image/png;base64," + Buffer.from(imageBytes).toString("base64");

  alert("Message encoded successfully");
});

// Decode

const decodeButton = document.getElementById("decodeButton");
const decodeChunkType = document.getElementById("decodeChunkType");
const decodeMessage = document.getElementById("decodeMessage");

decodeButton.addEventListener("click", () => {
  if (!imageBytes) {
    alert("Please upload an image first");
    return;
  }

  const chunkTypeValue = decodeChunkType.value || null;
  const message = wasm.decode(imageBytes, chunkTypeValue);

  if (message.length > 0) {
    decodeMessage.value = message.join("\n");
    alert("Message decoded successfully");
  } else {
    alert("No message found in the image");
  }
});

// Remove

const removeButton = document.getElementById("removeButton");
const removeChunkType = document.getElementById("removeChunkType");

removeButton.addEventListener("click", () => {
  if (!imageBytes) {
    alert("Please upload an image first");
    return;
  }

  const chunkTypeValue = removeChunkType.value || null;
  imageBytes = wasm.remove(imageBytes, chunkTypeValue);
  preview.src =
    "data:image/png;base64," + Buffer.from(imageBytes).toString("base64");

  alert("Chunk removed successfully");
});

// Print chunks

const printChunksButton = document.getElementById("printButton");
const printChunksOutput = document.getElementById("printChunks");

printChunksButton.addEventListener("click", () => {
  if (!imageBytes) {
    alert("Please upload an image first");
    return;
  }

  const chunks = wasm.print_chunks(imageBytes);
  printChunksOutput.value = chunks;
});
