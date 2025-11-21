use headless_chrome::{
    Browser, LaunchOptionsBuilder, protocol::cdp::Page::CaptureScreenshotFormatOption,
};
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Locate generated HTML
    let cwd = env::current_dir()?;
    let html_path = cwd.join("b50_slides.html");
    if !html_path.exists() {
        eprintln!("Error: 'b50_slides.html' not found. Run the generator first.");
        std::process::exit(1);
    }

    let file_url = format!("file://{}", html_path.to_string_lossy());

    // Launch headless Chrome once and reuse tab
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .idle_browser_timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to build LaunchOptions");

    let browser = match Browser::new(options) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to launch headless Chrome: {}", e);
            eprintln!(
                "Ensure Chrome/Chromium is installed or set PUPPETEER_EXECUTABLE_PATH/CHROME_EXECUTABLE."
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to launch headless Chrome: {}", e),
            )));
        }
    };

    let tab = browser.new_tab()?;

    // Navigate and wait
    tab.navigate_to(&file_url)?.wait_until_navigated()?;

    // Wait until records is available in page JS
    let mut records_count = 0usize;
    let wait_start = Instant::now();
    while wait_start.elapsed() < Duration::from_secs(6) {
        let val = tab.evaluate(
            "(function(){ return (typeof records !== 'undefined' && Array.isArray(records)) ? records.length : 0; })()",
            false,
        )?;
        if let Some(v) = val.value {
            if let Some(n) = v.as_u64() {
                records_count = n as usize;
                if records_count > 0 {
                    break;
                }
            }
        }
        sleep(Duration::from_millis(200));
    }

    if records_count == 0 {
        eprintln!("No records found in HTML; nothing to capture.");
        return Ok(());
    }

    // Ensure slides dir
    let slides_dir = cwd.join("slides");
    if !slides_dir.exists() {
        fs::create_dir_all(&slides_dir)?;
    }

    for i in 0..records_count {
        println!("Capturing slide_{}.png", i);

        // call page render(i) or scroll to item
        let _ = tab.evaluate(
            &format!(
                r#"(function(){{ if(typeof render === 'function') {{ try {{ render({}); }} catch(e){{}} }} else {{ var el = document.getElementById('item-{}'); if(el) el.scrollIntoView({{behavior:'auto', block:'center'}}); }} return true; }})();"#,
                i, i
            ),
            false,
        )?;

        // wait for animations and image load
        sleep(Duration::from_millis(800));

        // Wait until cover image is loaded (timeout 3s)
        let start = Instant::now();
        loop {
            let v = tab.evaluate(
                "(function(){ const img = document.getElementById('detail-cover'); return !img || img.complete; })()",
                false,
            )?;
            if let Some(val) = v.value {
                if val.as_bool().unwrap_or(true) {
                    break;
                }
            }
            if start.elapsed() > Duration::from_secs(3) {
                break;
            }
            sleep(Duration::from_millis(100));
        }

        // capture screenshot via CDP
        let png_data =
            tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;
        let out_path = slides_dir.join(format!("slide_{}.png", i));
        fs::write(out_path, png_data)?;

        // small delay between captures
        sleep(Duration::from_millis(200));
    }

    println!("Capture complete.");
    Ok(())
}
