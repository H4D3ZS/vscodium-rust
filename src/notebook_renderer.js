
export class NotebookRenderer {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
    }

    render(notebookContent) {
        if (!this.container) return;
        this.container.innerHTML = '';
        this.container.classList.remove('hidden');

        let data;
        try {
            data = JSON.parse(notebookContent);
        } catch (e) {
            this.container.innerText = "Error parsing notebook JSON: " + e.message;
            return;
        }

        // Render Metadata
        const metaDiv = document.createElement('div');
        metaDiv.className = 'notebook-metadata';
        if (data.metadata && data.metadata.kernelspec) {
            metaDiv.innerText = `Kernel: ${data.metadata.kernelspec.display_name} | Language: ${data.metadata.language_info?.name}`;
        }
        this.container.appendChild(metaDiv);

        // Render Cells
        if (data.cells && Array.isArray(data.cells)) {
            data.cells.forEach((cell, index) => {
                this.renderCell(cell, index);
            });
        }
    }

    renderCell(cell, index) {
        const cellDiv = document.createElement('div');
        cellDiv.className = `notebook-cell ${cell.cell_type}`;

        // Input Area
        const inputDiv = document.createElement('div');
        inputDiv.className = 'cell-input';

        // Prompt (In [x]:)
        const promptDiv = document.createElement('div');
        promptDiv.className = 'cell-prompt';
        if (cell.cell_type === 'code') {
            promptDiv.innerText = `In [${cell.execution_count || ' '}]:`;
        }
        inputDiv.appendChild(promptDiv);

        // Editor/Content
        const contentDiv = document.createElement('div');
        contentDiv.className = 'cell-content';
        const source = Array.isArray(cell.source) ? cell.source.join('') : (cell.source || '');

        if (cell.cell_type === 'markdown') {
            // TODO: Use a markdown parser (e.g. marked)
            // For now, simple text or innerHTML if we trust it (we shouldn't trust it blindly)
            contentDiv.style.whiteSpace = 'pre-wrap';
            contentDiv.innerText = source;
        } else {
            // Code
            contentDiv.innerText = source;
            // Ideally this would be a read-only Monaco/CodeMirror instance
        }
        inputDiv.appendChild(contentDiv);
        cellDiv.appendChild(inputDiv);

        // Output Area (only for code)
        if (cell.cell_type === 'code' && cell.outputs && cell.outputs.length > 0) {
            const outputDiv = document.createElement('div');
            outputDiv.className = 'cell-output-container';

            cell.outputs.forEach(output => {
                this.renderOutput(output, outputDiv);
            });
            cellDiv.appendChild(outputDiv);
        }

        this.container.appendChild(cellDiv);
    }

    renderOutput(output, container) {
        const outDiv = document.createElement('div');
        outDiv.className = 'cell-output';

        if (output.output_type === 'stream') {
            const text = Array.isArray(output.text) ? output.text.join('') : output.text;
            outDiv.className += ' stream ' + output.name; // stdout/stderr
            outDiv.innerText = text;
        } else if (output.output_type === 'execute_result' || output.output_type === 'display_data') {
            // Prioritize text/plain for now, can expand to image/png
            if (output.data['text/plain']) {
                const text = Array.isArray(output.data['text/plain']) ? output.data['text/plain'].join('') : output.data['text/plain'];
                outDiv.innerText = text;
            } else if (output.data['image/png']) {
                const img = document.createElement('img');
                img.src = 'data:image/png;base64,' + (Array.isArray(output.data['image/png']) ? output.data['image/png'].join('') : output.data['image/png']);
                outDiv.appendChild(img);
            }
        } else if (output.output_type === 'error') {
            outDiv.className += ' error';
            const trace = Array.isArray(output.traceback) ? output.traceback.join('\n') : output.traceback;
            outDiv.innerText = output.ename + ": " + output.evalue + "\n" + trace;
        }

        container.appendChild(outDiv);
    }
}
