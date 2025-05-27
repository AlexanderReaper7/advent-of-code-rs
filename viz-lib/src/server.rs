#[cfg(feature = "server")]
use warp::Filter;
#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
pub async fn start_visualization_server(port: u16) {
    println!("Starting visualization server on http://localhost:{}", port);
    
    // Serve static files from the web directory
    let static_files = warp::fs::dir("viz-web");
    
    // API endpoint to get trace data
    let traces = warp::path("api")
        .and(warp::path("traces"))
        .and(warp::fs::dir("traces"));
    
    // List all available traces
    let trace_list = warp::path("api")
        .and(warp::path("trace-list"))
        .and(warp::get())
        .map(|| {
            let traces_dir = Path::new("traces");
            if traces_dir.exists() {
                let entries: Vec<String> = std::fs::read_dir(traces_dir)
                    .unwrap()
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.extension()? == "json" {
                            path.file_stem()?.to_str().map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();
                warp::reply::json(&entries)
            } else {
                warp::reply::json(&Vec::<String>::new())
            }
        });
    
    let routes = static_files
        .or(traces)
        .or(trace_list);    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

#[cfg(feature = "server")]
pub fn create_web_interface() {
    // Create the web interface directory and files
    std::fs::create_dir_all("viz-web").unwrap();
    
    // Create the HTML file
    let html = include_str!("../web/index.html");
    std::fs::write("viz-web/index.html", html).unwrap();
    
    // Create the CSS file
    let css = include_str!("../web/style.css");
    std::fs::write("viz-web/style.css", css).unwrap();
    
    // Create the JavaScript file
    let js = include_str!("../web/script.js");
    std::fs::write("viz-web/script.js", js).unwrap();
    
    println!("Web interface created in viz-web/ directory");
}

#[cfg(feature = "server")]
pub async fn serve() {
    // Create web interface files
    create_web_interface();
    
    // Start the server on port 3031 to avoid conflicts
    start_visualization_server(3031).await;
}
