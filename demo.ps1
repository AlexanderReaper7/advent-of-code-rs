#!/usr/bin/env pwsh

# Code Execution Visualizer Demo Script
# This script demonstrates the complete visualization system

Write-Host "🎯 Code Execution Visualizer Demo" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Generate visualization traces
Write-Host "📊 Step 1: Generating visualization traces..." -ForegroundColor Yellow
Write-Host ""

Write-Host "   → Running Advent of Code Day 1 visualization (manual instrumentation)..."
cargo test test_visualization_generation --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "     ✅ Generated: traces/part1_visualized.json" -ForegroundColor Green
} else {
    Write-Host "     ❌ Failed to generate trace" -ForegroundColor Red
    exit 1
}

Write-Host "   → Running simple sort demo (proc macro)..."
cargo test test_proc_macro_visualization --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "     ✅ Generated: traces/simple_sort_demo.json" -ForegroundColor Green
} else {
    Write-Host "     ❌ Failed to generate trace" -ForegroundColor Red
    exit 1
}

Write-Host "   → Running detailed bubble sort demo (manual instrumentation)..."
cargo test test_detailed_visualization --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "     ✅ Generated: traces/detailed_sort_demo.json" -ForegroundColor Green
} else {
    Write-Host "     ❌ Failed to generate trace" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Step 2: Show trace files
Write-Host "📁 Step 2: Available trace files:" -ForegroundColor Yellow
Get-ChildItem -Path "traces" -Filter "*.json" | ForEach-Object {
    $size = [math]::Round($_.Length / 1KB, 1)
    Write-Host "   → $($_.Name) ($size KB)" -ForegroundColor White
}
Write-Host ""

# Step 3: Show trace preview
Write-Host "🔍 Step 3: Trace preview (detailed_sort_demo.json):" -ForegroundColor Yellow
$traceContent = Get-Content "traces/detailed_sort_demo.json" | ConvertFrom-Json
Write-Host "   Function: $($traceContent.function_name)" -ForegroundColor White
Write-Host "   Steps: $($traceContent.steps.Count)" -ForegroundColor White
Write-Host "   First few steps:" -ForegroundColor White
$traceContent.steps[0..4] | ForEach-Object {
    Write-Host "     $($_.step_id): $($_.description)" -ForegroundColor Gray
}
if ($traceContent.steps.Count -gt 5) {
    Write-Host "     ... and $($traceContent.steps.Count - 5) more steps" -ForegroundColor Gray
}
Write-Host ""

# Step 4: Start server
Write-Host "🚀 Step 4: Starting visualization server..." -ForegroundColor Yellow
Write-Host ""
Write-Host "Building with server features..." -ForegroundColor White
cargo build --features server --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to build server" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Server built successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "🌐 Starting web server on http://localhost:3030" -ForegroundColor Cyan
Write-Host ""
Write-Host "📖 Instructions:" -ForegroundColor Yellow
Write-Host "   1. The browser will open automatically" -ForegroundColor White
Write-Host "   2. Select a trace from the dropdown (try 'detailed_sort_demo')" -ForegroundColor White
Write-Host "   3. Click 'Load Trace'" -ForegroundColor White
Write-Host "   4. Use the controls to step through execution" -ForegroundColor White
Write-Host "   5. Try the 'Play' button for automatic playback" -ForegroundColor White
Write-Host ""
Write-Host "💡 Tip: The 'detailed_sort_demo' shows a complete bubble sort with swaps!" -ForegroundColor Cyan
Write-Host ""
Write-Host "Press Ctrl+C to stop the server" -ForegroundColor Yellow
Write-Host ""

# Start the server (this will block)
cargo run --features server viz
