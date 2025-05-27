class CodeVisualizer {
    constructor() {
        this.currentTrace = null;
        this.currentStep = 0;
        this.isPlaying = false;
        this.playInterval = null;
        
        this.initializeElements();
        this.bindEvents();
        this.loadAvailableTraces();
    }      initializeElements() {
        this.traceSelect = document.getElementById('traceSelect');
        this.loadTraceBtn = document.getElementById('loadTrace');
        this.functionName = document.getElementById('functionName');
        this.stepInfo = document.getElementById('stepInfo');
        this.prevStepBtn = document.getElementById('prevStep');
        this.nextStepBtn = document.getElementById('nextStep');
        this.playPauseBtn = document.getElementById('playPause');        this.stepDescription = document.getElementById('stepDescription');
        this.variablesView = document.getElementById('variablesView');
        this.arraysView = document.getElementById('arraysView');
        this.treesView = document.getElementById('treesView');
        this.metricsView = document.getElementById('metricsView');
        this.timeline = document.getElementById('timeline');
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
            traces.forEach(trace => {
                const option = document.createElement('option');
                option.value = trace;
                option.textContent = trace;
                this.traceSelect.appendChild(option);
            });
        } catch (error) {
            console.error('Error loading traces:', error);
        }
    }
    
    async loadSelectedTrace() {
        const selectedTrace = this.traceSelect.value;
        if (!selectedTrace) return;
        
        try {
            const response = await fetch(`/api/traces/${selectedTrace}.json`);
            this.currentTrace = await response.json();
            this.currentStep = 0;
              this.functionName.textContent = this.currentTrace.function_name;
            this.updateUI();
            this.renderTimeline();
            this.renderMetrics();
            this.enableControls();
        } catch (error) {
            console.error('Error loading trace:', error);
        }
    }
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
        const arraysContainer = document.getElementById('arraysView');
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
                
                // Add special styling for highlighted indices
                if (arrayState.highlighted_indices && arrayState.highlighted_indices.includes(index)) {
                    element.classList.add('highlighted');
                }
                
                // Add special styling for comparison indices
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
            
            arrayDiv.appendChild(arrayContainer);            arraysContainer.appendChild(arrayDiv);
        });
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
        const timelineSteps = this.timeline.querySelectorAll('.timeline-step');
        timelineSteps.forEach((step, index) => {
            step.classList.remove('active', 'completed');
            if (index === this.currentStep) {
                step.classList.add('active');
            } else if (index < this.currentStep) {
                step.classList.add('completed');
            }
        });
    }
    
    updateStepButtons() {
        if (!this.currentTrace) return;
        
        this.prevStepBtn.disabled = this.currentStep === 0;
        this.nextStepBtn.disabled = this.currentStep >= this.currentTrace.steps.length - 1;
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
        }, 1000); // Advance every second
    }
    
    pause() {
        this.isPlaying = false;
        this.playPauseBtn.textContent = 'Play';
        
        if (this.playInterval) {
            clearInterval(this.playInterval);
            this.playInterval = null;
        }
    }
    
    renderTrees(trees) {
        const treesContainer = document.getElementById('treesView');
        if (!treesContainer) return;
        
        if (!trees || Object.keys(trees).length === 0) {
            treesContainer.innerHTML = '<p>No trees to display</p>';
            return;
        }
        
        treesContainer.innerHTML = '';
        Object.entries(trees).forEach(([name, treeState]) => {
            const treeDiv = document.createElement('div');
            treeDiv.className = 'tree-visualization';
            
            const treeTitle = document.createElement('h4');
            treeTitle.textContent = `${name} (${treeState.tree_type})`;
            treeDiv.appendChild(treeTitle);
            
            if (treeState.root) {
                const treeContainer = document.createElement('div');
                treeContainer.className = 'tree-container';
                
                // Calculate tree layout
                const layout = this.calculateTreeLayout(treeState.root);
                const svgElement = this.createTreeSVG(treeState, layout);
                
                treeContainer.appendChild(svgElement);
                treeDiv.appendChild(treeContainer);
            } else {
                const emptyMessage = document.createElement('p');
                emptyMessage.textContent = 'Empty tree';
                emptyMessage.className = 'empty-tree-message';
                treeDiv.appendChild(emptyMessage);
            }
            
            treesContainer.appendChild(treeDiv);
        });
    }
    
    calculateTreeLayout(rootNode) {
        const layout = { nodes: [], connections: [] };
        const nodeWidth = 60;
        const nodeHeight = 40;
        const levelHeight = 80;
        
        // First pass: assign levels and calculate subtree widths
        const assignLevels = (node, level, x) => {
            if (!node) return { width: 0, minX: x, maxX: x };
            
            const nodeInfo = {
                id: node.id,
                value: node.value,
                level: level,
                x: x,
                y: level * levelHeight + 50,
                highlighted: node.highlighted || false,
                comparing: node.comparing || false
            };
            
            let totalWidth = nodeWidth;
            let currentX = x;
            
            // Calculate left subtree
            if (node.left) {
                const leftResult = assignLevels(node.left, level + 1, currentX);
                totalWidth += leftResult.width + 20; // 20px spacing
                layout.connections.push({
                    from: { x: x + nodeWidth/2, y: nodeInfo.y + nodeHeight/2 },
                    to: { x: leftResult.nodes[0]?.x + nodeWidth/2 || currentX, y: (level + 1) * levelHeight + 50 + nodeHeight/2 }
                });
            }
            
            // Position current node
            nodeInfo.x = currentX + (totalWidth - nodeWidth) / 2;
            layout.nodes.push(nodeInfo);
            
            // Calculate right subtree
            if (node.right) {
                const rightX = currentX + totalWidth + 20;
                const rightResult = assignLevels(node.right, level + 1, rightX);
                totalWidth += rightResult.width + 20;
                layout.connections.push({
                    from: { x: nodeInfo.x + nodeWidth/2, y: nodeInfo.y + nodeHeight/2 },
                    to: { x: rightResult.nodes[0]?.x + nodeWidth/2 || rightX, y: (level + 1) * levelHeight + 50 + nodeHeight/2 }
                });
            }
            
            return { width: totalWidth, nodes: [nodeInfo] };
        };
        
        if (rootNode) {
            assignLevels(rootNode, 0, 50);
        }
        
        return layout;
    }
    
    createTreeSVG(treeState, layout) {
        const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
        svg.setAttribute('class', 'tree-svg');
        
        // Calculate SVG dimensions
        const maxX = Math.max(...layout.nodes.map(n => n.x)) + 100;
        const maxY = Math.max(...layout.nodes.map(n => n.y)) + 100;
        svg.setAttribute('viewBox', `0 0 ${maxX} ${maxY}`);
        svg.setAttribute('width', '100%');
        svg.setAttribute('height', Math.min(maxY, 500));
        
        // Draw connections first (so they appear behind nodes)
        layout.connections.forEach(conn => {
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('x1', conn.from.x);
            line.setAttribute('y1', conn.from.y);
            line.setAttribute('x2', conn.to.x);
            line.setAttribute('y2', conn.to.y);
            line.setAttribute('class', 'tree-connection');
            svg.appendChild(line);
        });
        
        // Draw nodes
        layout.nodes.forEach(node => {
            // Node background circle/rectangle
            const nodeGroup = document.createElementNS('http://www.w3.org/2000/svg', 'g');
            nodeGroup.setAttribute('class', 'tree-node-group');
            
            const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
            circle.setAttribute('cx', node.x + 30); // Center of 60px width
            circle.setAttribute('cy', node.y + 20); // Center of 40px height
            circle.setAttribute('r', 25);
            
            let nodeClass = 'tree-node';
            if (node.highlighted) nodeClass += ' highlighted';
            if (node.comparing) nodeClass += ' comparing';
            
            circle.setAttribute('class', nodeClass);
            nodeGroup.appendChild(circle);
            
            // Node text
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('x', node.x + 30);
            text.setAttribute('y', node.y + 25);
            text.setAttribute('text-anchor', 'middle');
            text.setAttribute('class', 'tree-node-text');
            text.textContent = node.value;
            nodeGroup.appendChild(text);
            
            svg.appendChild(nodeGroup);
        });
        
        // Add traversal path if present
        if (treeState.traversal_path && treeState.traversal_path.length > 0) {
            this.drawTraversalPath(svg, layout, treeState.traversal_path);
        }
        
        return svg;
    }
    
    drawTraversalPath(svg, layout, traversalPath) {
        const pathGroup = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        pathGroup.setAttribute('class', 'traversal-path');
        
        for (let i = 0; i < traversalPath.length - 1; i++) {
            const fromNodeId = traversalPath[i];
            const toNodeId = traversalPath[i + 1];
            
            const fromNode = layout.nodes.find(n => n.id === fromNodeId);
            const toNode = layout.nodes.find(n => n.id === toNodeId);
            
            if (fromNode && toNode) {
                const pathLine = document.createElementNS('http://www.w3.org/2000/svg', 'line');
                pathLine.setAttribute('x1', fromNode.x + 30);
                pathLine.setAttribute('y1', fromNode.y + 20);
                pathLine.setAttribute('x2', toNode.x + 30);
                pathLine.setAttribute('y2', toNode.y + 20);
                pathLine.setAttribute('class', 'traversal-line');
                pathLine.setAttribute('stroke-dasharray', '5,5');
                pathGroup.appendChild(pathLine);
            }
        }
        
        svg.appendChild(pathGroup);
    }
}

// Initialize the visualizer when the page loads
document.addEventListener('DOMContentLoaded', () => {
    new CodeVisualizer();
});
