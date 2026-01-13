// Importar funciones de Tauri v2 usando ES modules
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/api/dialog';

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

// === EVENTOS (cuando el usuario hace algo) ===

// 1. Click en la zona de drop → abrir selector de archivos
dropZone.addEventListener('click', async () => {
  try {
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Imágenes',
        extensions: ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp']
      }],
      title: 'Selecciona imágenes para comprimir'
    });

    if (selected) {
      selectedFiles = Array.isArray(selected) ? selected : [selected];
      fileCount.textContent = `${selectedFiles.length} imagen(es) seleccionada(s)`;
      checkReadyToProcess();
    }
  } catch (error) {
    console.error('Error al seleccionar archivos:', error);
    alert('Error al abrir selector: ' + error);
  }
});

// 2. Click en botón "Elegir carpeta"
selectDirBtn.addEventListener('click', async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Selecciona carpeta de salida'
    });

    if (selected) {
      outputDirectory = selected;
      outputDirInput.value = selected;
      checkReadyToProcess();
    }
  } catch (error) {
    console.error('Error al seleccionar carpeta:', error);
    alert('Error al abrir selector de carpeta: ' + error);
  }
});

// 3. Click en botón "Procesar"
processBtn.addEventListener('click', async () => {
  if (selectedFiles.length === 0 || !outputDirectory) {
    alert('Selecciona imágenes y carpeta de salida');
    return;
  }

  // Deshabilitar botón mientras procesa
  processBtn.disabled = true;
  processBtn.textContent = 'Procesando...';

  try {
    // Obtener valores de la UI
    const quality = document.querySelector('input[name="quality"]:checked').value;
    const format = document.getElementById('format').value;
    const widthInput = document.getElementById('width').value;
    const width = widthInput ? parseInt(widthInput) : null;

    // === LLAMADA AL BACKEND RUST ===
    const result = await invoke('process_images_command', {
      paths: selectedFiles,
      quality: quality,
      format: format,
      privacy: 'keep_all',
      width: width,
      outputDir: outputDirectory
    });

    // Mostrar resultados
    showResults(result);

  } catch (error) {
    alert(`Error al procesar: ${error}`);
    console.error(error);
  } finally {
    // Rehabilitar botón
    processBtn.disabled = false;
    processBtn.textContent = 'Procesar imágenes';
  }
});

// === FUNCIONES AUXILIARES ===

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