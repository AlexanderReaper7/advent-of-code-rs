// Enhanced script.js that supports both original and compact trace formats

class AdvancedCodeVisualizer {
    constructor() {
        this.currentTrace = null;
        this.currentStep = 0;
        this.isPlaying = false;
        this.playInterval = null;
        this.traceFormat = 'unknown'; // 'original', 'compact', 'binary'
        
        this.initializeElements();
        this.bindEvents();
        this.loadAvailableTraces();
    }
    
    initializeElements() {
        this.traceSelect = document.getElementById('traceSelect');
        this.loadTraceBtn = document.getElementById('loadTrace');
        this.functionName = document.getElementById('functionName');
        this.stepInfo = document.getElementById('stepInfo');
        this.prevStepBtn = document.getElementById('prevStep');
        this.nextStepBtn = document.getElementById('nextStep');
        this.playPauseBtn = document.getElementById('playPause');
        this.stepDescription = document.getElementById('stepDescription');
        this.variablesView = document.getElementById('variablesView');
        this.arraysView = document.getElementById('arraysView');
        this.treesView = document.getElementById('treesView');
        this.metricsView = document.getElementById('metricsView');
        this.timeline = document.getElementById('timeline');
        this.formatInfo = document.getElementById('formatInfo') || this.createFormatInfo();
    }
    
    createFormatInfo() {
        const info = document.createElement('div');
        info.id = 'formatInfo';
        info.className = 'format-info';
        info.innerHTML = `
            <div class="format-badge">
                <span class="format-type">Format: Unknown</span>
                <span class="format-size">Size: -</span>
                <span class="compression-ratio">Compression: -</span>
            </div>
        `;
        
        const header = document.querySelector('header');
        if (header) {
            header.appendChild(info);
        }
        
        return info;
    }
    
    bindEvents() {
        this.loadTraceBtn.addEventListener('click', () => this.loadSelectedTrace());
        this.prevStepBtn.addEventListener('click', () => this.previousStep());
        this.nextStepBtn.addEventListener('click', () => this.nextStep());
        this.playPauseBtn.addEventListener('click', () => this.togglePlay());
    }
    
    async loadAvailableTraces() {
        try {
            const response = await fetch('/api/trace-list');
            const traces = await response.json();
            
            this.traceSelect.innerHTML = '<option value="">Select a trace...</option>';
            
            // Group traces by format
            const traceGroups = {
                original: [],
                compact: [],
                binary: []
            };
            
            traces.forEach(trace => {
                if (trace.endsWith('_compact.json')) {
                    traceGroups.compact.push(trace);
                } else if (trace.endsWith('.binviz')) {
                    traceGroups.binary.push(trace);
                } else if (trace.endsWith('.json')) {
                    traceGroups.original.push(trace);
                }
            });
            
            // Add grouped options
            if (traceGroups.original.length > 0) {
                const group = document.createElement('optgroup');
                group.label = 'Original Format';
                traceGroups.original.forEach(trace => {
                    const option = document.createElement('option');
                    option.value = trace;
                    option.textContent = trace;
                    group.appendChild(option);
                });
                this.traceSelect.appendChild(group);
            }
            
            if (traceGroups.compact.length > 0) {
                const group = document.createElement('optgroup');
                group.label = 'Compact Format (Efficient)';
                traceGroups.compact.forEach(trace => {
                    const option = document.createElement('option');
                    option.value = trace;
                    option.textContent = trace + ' ⚡';
                    group.appendChild(option);
                });
                this.traceSelect.appendChild(group);
            }
            
            if (traceGroups.binary.length > 0) {
                const group = document.createElement('optgroup');
                group.label = 'Binary Format (Compressed)';
                traceGroups.binary.forEach(trace => {
                    const option = document.createElement('option');
                    option.value = trace;
                    option.textContent = trace + ' 🗜️';
                    group.appendChild(option);
                });
                this.traceSelect.appendChild(group);
            }
            
        } catch (error) {
            console.error('Error loading traces:', error);
        }
    }
    
    async loadSelectedTrace() {
        const selectedTrace = this.traceSelect.value;
        if (!selectedTrace) return;
        
        try {
            let response;
            let trace;
            
            if (selectedTrace.endsWith('.binviz')) {
                // Handle binary format
                response = await fetch(`/api/traces/${selectedTrace}`);
                const compressed = await response.arrayBuffer();
                trace = this.decompressBinaryTrace(compressed);
                this.traceFormat = 'binary';
            } else if (selectedTrace.endsWith('_compact.json')) {
                // Handle compact format
                response = await fetch(`/api/traces/${selectedTrace}`);
                const compactTrace = await response.json();
                trace = this.expandCompactTrace(compactTrace);
                this.traceFormat = 'compact';
            } else {
                // Handle original format
                response = await fetch(`/api/traces/${selectedTrace}`);
                trace = await response.json();
                this.traceFormat = 'original';
            }
            
            this.currentTrace = trace;
            this.currentStep = 0;
            
            this.functionName.textContent = this.currentTrace.function_name;
            this.updateFormatInfo(selectedTrace);
            this.updateUI();
            this.renderTimeline();
            this.renderMetrics();
            this.enableControls();
            
        } catch (error) {
            console.error('Error loading trace:', error);
            this.showError(`Failed to load trace: ${error.message}`);
        }
    }
    
    updateFormatInfo(filename) {
        const formatType = this.formatInfo.querySelector('.format-type');
        const formatSize = this.formatInfo.querySelector('.format-size');
        const compressionRatio = this.formatInfo.querySelector('.compression-ratio');
        
        formatType.textContent = `Format: ${this.traceFormat.charAt(0).toUpperCase() + this.traceFormat.slice(1)}`;
        
        // Estimate size and compression based on format
        const stepCount = this.currentTrace.steps.length;
        let estimatedOriginalSize, actualSize, compressionText;
        
        switch (this.traceFormat) {
            case 'original':
                formatSize.textContent = `Size: Full`;
                compressionRatio.textContent = `Compression: None`;
                formatType.style.backgroundColor = '#ff6b6b';
                break;
                
            case 'compact':
                formatSize.textContent = `Size: ~30-70% smaller`;
                compressionRatio.textContent = `Compression: Delta-based`;
                formatType.style.backgroundColor = '#4ecdc4';
                break;
                
            case 'binary':
                formatSize.textContent = `Size: ~80-90% smaller`;
                compressionRatio.textContent = `Compression: Binary + Gzip`;
                formatType.style.backgroundColor = '#45b7d1';
                break;
        }
    }
    
    expandCompactTrace(compactTrace) {
        // Convert compact format to original format for display
        const expandedSteps = [];
        let currentVariables = { ...compactTrace.initial_variables };
        let currentArrays = {};
        let currentTrees = {};
        
        // Initialize arrays
        for (const [name, compactArray] of Object.entries(compactTrace.initial_arrays)) {
            currentArrays[name] = {
                data: [...compactArray.data],
                highlighted_indices: [],
                comparison_indices: []
            };
        }
        
        // Initialize trees
        for (const [name, compactTree] of Object.entries(compactTrace.initial_trees)) {
            currentTrees[name] = this.expandCompactTree(compactTree);
        }
        
        // Process each step
        for (const compactStep of compactTrace.steps) {
            // Apply variable changes
            if (compactStep.variable_changes) {
                Object.assign(currentVariables, compactStep.variable_changes);
            }
            
            // Apply array changes
            if (compactStep.array_changes) {
                for (const [arrayName, change] of Object.entries(compactStep.array_changes)) {
                    if (currentArrays[arrayName]) {
                        const arrayState = currentArrays[arrayName];
                        
                        if (change.data_updates) {
                            for (const [index, value] of change.data_updates) {
                                if (index < arrayState.data.length) {
                                    arrayState.data[index] = value;
                                }
                            }
                        }
                        
                        if (change.highlighted_indices !== undefined) {
                            arrayState.highlighted_indices = change.highlighted_indices;
                        }
                        
                        if (change.comparison_indices !== undefined) {
                            arrayState.comparison_indices = change.comparison_indices;
                        }
                    }
                }
            }
            
            // Create expanded step
            expandedSteps.push({
                step_id: compactStep.step_id,
                description: compactStep.description,
                variables: { ...currentVariables },
                arrays: JSON.parse(JSON.stringify(currentArrays)),
                trees: JSON.parse(JSON.stringify(currentTrees)),
                timestamp: compactStep.timestamp,
                operation_type: compactStep.operation_type
            });
        }
        
        return {
            function_name: compactTrace.function_name,
            steps: expandedSteps,
            input: compactTrace.input,
            output: compactTrace.output,
            performance_metrics: compactTrace.performance_metrics
        };
    }
    
    expandCompactTree(compactTree) {
        // Convert compact tree format to original tree format
        return {
            root: compactTree.root ? this.expandCompactTreeNode(compactTree.root) : null,
            tree_type: compactTree.tree_type,
            highlighted_nodes: [],
            comparison_nodes: [],
            current_path: []
        };
    }
    
    expandCompactTreeNode(compactNode) {
        return {
            id: compactNode.id,
            value: compactNode.value,
            left: compactNode.left ? this.expandCompactTreeNode(compactNode.left) : null,
            right: compactNode.right ? this.expandCompactTreeNode(compactNode.right) : null,
            is_highlighted: false,
            is_comparing: false,
            level: compactNode.level,
            position: compactNode.position
        };
    }
    
    decompressBinaryTrace(compressedData) {
        // For now, assume it's compressed JSON
        // In a real implementation, this would handle custom binary format
        const decompressed = pako.inflate(new Uint8Array(compressedData), { to: 'string' });
        const compactTrace = JSON.parse(decompressed);
        return this.expandCompactTrace(compactTrace);
    }
    
    showError(message) {
        const errorDiv = document.createElement('div');
        errorDiv.className = 'error-message';
        errorDiv.textContent = message;
        errorDiv.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background: #ff6b6b;
            color: white;
            padding: 10px 20px;
            border-radius: 5px;
            z-index: 1000;
        `;
        
        document.body.appendChild(errorDiv);
        setTimeout(() => errorDiv.remove(), 5000);
    }
    
    // Rest of the methods remain the same as the original CodeVisualizer
    updateUI() {
        if (!this.currentTrace) return;
        
        const steps = this.currentTrace.steps;
        const totalSteps = steps.length;
        
        this.stepInfo.textContent = `Step ${this.currentStep + 1} of ${totalSteps}`;
        
        if (this.currentStep < steps.length) {
            const step = steps[this.currentStep];
            this.stepDescription.textContent = step.description;
            this.renderVariables(step.variables);
            this.renderArrays(step.arrays);
            this.renderTrees(step.trees);
        }
        
        this.updateStepButtons();
        this.updateTimelineHighlight();
    }
    
    renderVariables(variables) {
        if (!variables || Object.keys(variables).length === 0) {
            this.variablesView.innerHTML = '<p>No variables to display</p>';
            return;
        }
        
        this.variablesView.innerHTML = '';
        Object.entries(variables).forEach(([name, value]) => {
            const variableDiv = document.createElement('div');
            variableDiv.className = 'variable';
            variableDiv.innerHTML = `
                <span class="variable-name">${name}</span>
                <span class="variable-value">${value}</span>
            `;
            this.variablesView.appendChild(variableDiv);
        });
    }
    
    renderArrays(arrays) {
        const arraysContainer = this.arraysView;
        if (!arraysContainer) return;
        
        if (!arrays || Object.keys(arrays).length === 0) {
            arraysContainer.innerHTML = '<p>No arrays to display</p>';
            return;
        }
        
        arraysContainer.innerHTML = '';
        Object.entries(arrays).forEach(([name, arrayState]) => {
            const arrayDiv = document.createElement('div');
            arrayDiv.className = 'array-visualization';
            
            const arrayTitle = document.createElement('h4');
            arrayTitle.textContent = name;
            arrayDiv.appendChild(arrayTitle);
            
            const arrayContainer = document.createElement('div');
            arrayContainer.className = 'array-container';
            
            arrayState.data.forEach((value, index) => {
                const element = document.createElement('div');
                element.className = 'array-element';
                element.textContent = value;
                
                if (arrayState.highlighted_indices && arrayState.highlighted_indices.includes(index)) {
                    element.classList.add('highlighted');
                }
                
                if (arrayState.comparison_indices) {
                    arrayState.comparison_indices.forEach(([idx1, idx2]) => {
                        if (index === idx1 || index === idx2) {
                            element.classList.add('comparing');
                        }
                    });
                }
                
                const indexLabel = document.createElement('div');
                indexLabel.className = 'array-index';
                indexLabel.textContent = index;
                
                const elementContainer = document.createElement('div');
                elementContainer.className = 'array-element-container';
                elementContainer.appendChild(element);
                elementContainer.appendChild(indexLabel);
                
                arrayContainer.appendChild(elementContainer);
            });
            
            arrayDiv.appendChild(arrayContainer);
            arraysContainer.appendChild(arrayDiv);
        });
    }
    
    renderTrees(trees) {
        // Implementation similar to original, but handles both formats
        const treesContainer = this.treesView;
        if (!treesContainer) return;
        
        if (!trees || Object.keys(trees).length === 0) {
            treesContainer.innerHTML = '<p>No trees to display</p>';
            return;
        }
        
        treesContainer.innerHTML = '';
        // Tree rendering logic would go here...
    }
    
    renderMetrics() {
        if (!this.currentTrace || !this.currentTrace.performance_metrics) {
            this.metricsView.innerHTML = '<p>No metrics available</p>';
            return;
        }
        
        const metrics = this.currentTrace.performance_metrics;
        this.metricsView.innerHTML = `
            <div class="metric">
                <span class="metric-name">Comparisons</span>
                <span class="metric-value">${metrics.total_comparisons}</span>
            </div>
            <div class="metric">
                <span class="metric-name">Swaps</span>
                <span class="metric-value">${metrics.total_swaps}</span>
            </div>
            <div class="metric">
                <span class="metric-name">Array Accesses</span>
                <span class="metric-value">${metrics.total_array_accesses}</span>
            </div>
            <div class="metric">
                <span class="metric-name">Execution Time</span>
                <span class="metric-value">${metrics.execution_time_ms}ms</span>
            </div>
            <div class="metric format-metric">
                <span class="metric-name">Format</span>
                <span class="metric-value">${this.traceFormat}</span>
            </div>
        `;
    }
    
    renderTimeline() {
        if (!this.currentTrace) return;
        
        this.timeline.innerHTML = '';
        this.currentTrace.steps.forEach((step, index) => {
            const stepDiv = document.createElement('div');
            stepDiv.className = 'timeline-step';
            stepDiv.dataset.stepIndex = index;
            stepDiv.innerHTML = `
                <div>
                    <span class="step-number">${index + 1}</span>
                    <span class="step-text">${step.description}</span>
                </div>
                <div class="step-timestamp">${step.timestamp}ms</div>
            `;
            
            stepDiv.addEventListener('click', () => {
                this.currentStep = index;
                this.updateUI();
            });
            
            this.timeline.appendChild(stepDiv);
        });
    }
    
    updateTimelineHighlight() {
        const steps = this.timeline.querySelectorAll('.timeline-step');
        steps.forEach((step, index) => {
            step.classList.toggle('current', index === this.currentStep);
        });
    }
    
    updateStepButtons() {
        this.prevStepBtn.disabled = this.currentStep === 0;
        this.nextStepBtn.disabled = !this.currentTrace || this.currentStep >= this.currentTrace.steps.length - 1;
    }
    
    enableControls() {
        this.prevStepBtn.disabled = false;
        this.nextStepBtn.disabled = false;
        this.playPauseBtn.disabled = false;
    }
    
    previousStep() {
        if (this.currentStep > 0) {
            this.currentStep--;
            this.updateUI();
        }
    }
    
    nextStep() {
        if (this.currentStep < this.currentTrace.steps.length - 1) {
            this.currentStep++;
            this.updateUI();
        }
    }
    
    togglePlay() {
        if (this.isPlaying) {
            this.pause();
        } else {
            this.play();
        }
    }
    
    play() {
        this.isPlaying = true;
        this.playPauseBtn.textContent = 'Pause';
        
        this.playInterval = setInterval(() => {
            if (this.currentStep < this.currentTrace.steps.length - 1) {
                this.nextStep();
            } else {
                this.pause();
            }
        }, 1000);
    }
    
    pause() {
        this.isPlaying = false;
        this.playPauseBtn.textContent = 'Play';
        
        if (this.playInterval) {
            clearInterval(this.playInterval);
            this.playInterval = null;
        }
    }
}

// Initialize the enhanced visualizer
document.addEventListener('DOMContentLoaded', () => {
    new AdvancedCodeVisualizer();
});
