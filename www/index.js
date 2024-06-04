import * as wasm from "wasm-pngme";

var upload = document.getElementById("upload");
var preview = document.getElementById("preview");
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

var encodeButton = document.getElementById("encode");
var chunkType = document.getElementById("chunkType");
var message = document.getElementById("message");

encodeButton.addEventListener("click", () => {
  if (!imageBytes) {
    alert("Please upload an image first");
    return;
  }

  const chunkTypeValue = chunkType.value;
  const messageValue = message.value;

  const newImageBytes = wasm.encode(imageBytes, chunkTypeValue, messageValue);
  preview.src =
    "data:image/png;base64," + Buffer.from(newImageBytes).toString("base64");
});
