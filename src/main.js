// Tauri v2 API without imports
const invoke = window.__TAURI_INTERNALS__.invoke;

// Global variables
let selectedFiles = [];
let outputDirectory = '';

// HTML element references
const dropZone = document.getElementById('dropZone');
const fileCount = document.getElementById('fileCount');
const processBtn = document.getElementById('processBtn');
const selectDirBtn = document.getElementById('selectDirBtn');
const outputDirInput = document.getElementById('outputDir');
const results = document.getElementById('results');
const clearBtn = document.getElementById('clearBtn');

// Click on zone to select files
dropZone.addEventListener('click', async () => {
  try {
    const selected = await invoke('plugin:dialog|open', {
      options: {
        multiple: true,
        filters: [{
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp']
        }]
      }
    });

    if (selected) {
      selectedFiles = Array.isArray(selected) ? selected : [selected];
      fileCount.textContent = `${selectedFiles.length} image(s) selected`;
      checkReadyToProcess();
    }
  } catch (error) {
    console.error('Error:', error);
    alert('Error: ' + error);
  }
});

// Click on select folder button
selectDirBtn.addEventListener('click', async () => {
  try {
    const selected = await invoke('plugin:dialog|open', {
      options: {
        directory: true
      }
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

// Click on process button
processBtn.addEventListener('click', async () => {
  if (selectedFiles.length === 0 || !outputDirectory) {
    alert('Select images and output folder');
    return;
  }

  processBtn.disabled = true;
  processBtn.textContent = 'Processing...';

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
    alert(`Processing error: ${error}`);
    console.error(error);
  } finally {
    processBtn.disabled = false;
    processBtn.textContent = 'Process Images';
  }
});

// Click on clear button
clearBtn.addEventListener('click', () => {
  selectedFiles = [];
  outputDirectory = '';
  fileCount.textContent = '';
  outputDirInput.value = '';
  results.hidden = true;
  processBtn.disabled = true;
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