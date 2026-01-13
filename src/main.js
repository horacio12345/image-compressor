// API de Tauri v2 sin imports
const invoke = window.__TAURI_INTERNALS__.invoke;

// Variables globales
let selectedFiles = [];
let outputDirectory = '';

// Referencias a elementos HTML
const dropZone = document.getElementById('dropZone');
const fileCount = document.getElementById('fileCount');
const processBtn = document.getElementById('processBtn');
const selectDirBtn = document.getElementById('selectDirBtn');
const outputDirInput = document.getElementById('outputDir');
const results = document.getElementById('results');

// Click en zona para seleccionar archivos
dropZone.addEventListener('click', async () => {
  try {
    const selected = await invoke('plugin:dialog|open', {
      multiple: true,
      filters: [{
        name: 'Imágenes',
        extensions: ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp']
      }]
    });

    if (selected) {
      selectedFiles = Array.isArray(selected) ? selected : [selected];
      fileCount.textContent = `${selectedFiles.length} imagen(es) seleccionada(s)`;
      checkReadyToProcess();
    }
  } catch (error) {
    console.error('Error:', error);
    alert('Error: ' + error);
  }
});

// Click en botón elegir carpeta
selectDirBtn.addEventListener('click', async () => {
  try {
    const selected = await invoke('plugin:dialog|open', {
      directory: true
    });

    if (selected) {
      outputDirectory = selected;
      outputDirInput.value = selected;
      checkReadyToProcess();
    }
  } catch (error) {
    console.error('Error:', error);
    alert('Error: ' + error);
  }
});

// Click en procesar
processBtn.addEventListener('click', async () => {
  if (selectedFiles.length === 0 || !outputDirectory) {
    alert('Selecciona imágenes y carpeta de salida');
    return;
  }

  processBtn.disabled = true;
  processBtn.textContent = 'Procesando...';

  try {
    const quality = document.querySelector('input[name="quality"]:checked').value;
    const format = document.getElementById('format').value;
    const widthInput = document.getElementById('width').value;
    const width = widthInput ? parseInt(widthInput) : null;

    const result = await invoke('process_images_command', {
      paths: selectedFiles,
      quality: quality,
      format: format,
      privacy: 'keep_all',
      width: width,
      outputDir: outputDirectory
    });

    showResults(result);

  } catch (error) {
    alert(`Error al procesar: ${error}`);
    console.error(error);
  } finally {
    processBtn.disabled = false;
    processBtn.textContent = 'Procesar imágenes';
  }
});

function checkReadyToProcess() {
  const ready = selectedFiles.length > 0 && outputDirectory !== '';
  processBtn.disabled = !ready;
}

function showResults(result) {
  document.getElementById('totalCount').textContent = result.total_images;
  document.getElementById('successCount').textContent = result.successful;
  document.getElementById('failedCount').textContent = result.failed;
  results.hidden = false;
}