# Code Execution Visualizer - Enhanced Demo
# This script demonstrates the enhanced array visualization features

Write-Host "🚀 Code Execution Visualizer - Enhanced Features Demo" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Gray

# Step 1: Generate enhanced visualization traces
Write-Host "`n📊 Step 1: Generating Enhanced Visualizations..." -ForegroundColor Yellow

Write-Host "  • Bubble Sort with Array Visualization..."
cargo test test_enhanced_bubble_sort --quiet

Write-Host "  • Selection Sort with Array Visualization..."  
cargo test test_enhanced_selection_sort --quiet

Write-Host "  • Binary Search Visualization..."
cargo test test_binary_search --quiet

Write-Host "  • BFS Pathfinding Algorithm..."
cargo test test_bfs_pathfinder --quiet

Write-Host "  • Dijkstra's Algorithm..."
cargo test test_dijkstra_algorithm --quiet

# Step 2: Show available trace files
Write-Host "`n📁 Step 2: Available Visualization Traces:" -ForegroundColor Yellow
Get-ChildItem -Path "traces" -Filter "*.json" | ForEach-Object {
    $fileSize = [math]::Round($_.Length / 1KB, 1)
    Write-Host "  • $($_.Name) ($fileSize KB)" -ForegroundColor Green
}

# Step 3: Start the visualization server
Write-Host "`n🌐 Step 3: Starting Visualization Server..." -ForegroundColor Yellow
Write-Host "  Server will start on http://localhost:3030" -ForegroundColor Cyan

Write-Host "`n🎯 Enhanced Features Available:" -ForegroundColor Magenta
Write-Host "  ✅ Array element highlighting during operations" -ForegroundColor Green
Write-Host "  ✅ Comparison visualization with color coding" -ForegroundColor Green  
Write-Host "  ✅ Swap animations and state tracking" -ForegroundColor Green
Write-Host "  ✅ Real-time variable monitoring" -ForegroundColor Green
Write-Host "  ✅ Step-by-step execution timeline" -ForegroundColor Green
Write-Host "  ✅ Operation type categorization" -ForegroundColor Green
Write-Host "  ✅ Pathfinding visualization with maze display" -ForegroundColor Green
Write-Host "  ✅ Graph algorithm visualization" -ForegroundColor Green

Write-Host "`n📚 Recommended Viewing Order:" -ForegroundColor Cyan
Write-Host "  1. enhanced_bubble_sort.json - See sorting with highlights" -ForegroundColor White
Write-Host "  2. binary_search.json - Watch binary search in action" -ForegroundColor White
Write-Host "  3. enhanced_selection_sort.json - Compare sorting algorithms" -ForegroundColor White
Write-Host "  4. bfs_pathfinder.json - Maze solving visualization" -ForegroundColor White
Write-Host "  5. dijkstra_algorithm.json - Shortest path finding" -ForegroundColor White

Write-Host "`n🎮 Web Interface Controls:" -ForegroundColor Cyan
Write-Host "  • Use Previous/Next buttons to step through execution" -ForegroundColor White
Write-Host "  • Click Play to auto-advance through steps" -ForegroundColor White
Write-Host "  • Click any step in the timeline to jump to it" -ForegroundColor White
Write-Host "  • Watch arrays change color during operations" -ForegroundColor White
Write-Host "  • Monitor variable values in real-time" -ForegroundColor White

Write-Host "`n⚡ Starting server..." -ForegroundColor Yellow
Write-Host "Press Ctrl+C to stop the server when done." -ForegroundColor Gray

# Start the server
cargo run --features server viz
