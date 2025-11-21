use crate::models::B50Project;
use std::fs::File;
use std::io::Write;

pub struct ScriptGenerator;

impl ScriptGenerator {
    pub fn generate(project: &B50Project) -> std::io::Result<()> {
        let total = project.records.len();
        let last = if total == 0 { 0 } else { total - 1 };

        let script_content = format!(
            r#"#!/usr/bin/env bash
set -euo pipefail
# Paradigm B50 Render Script
# Generated for {player}
# Note: Since the UI is now a Single Page App, ensure you capture screenshots
# matching the timing or use a screen recorder.

TOTAL={total}
LAST={last}
echo "Checking slides directory for $TOTAL slides..."
missing=false
if [ ! -d "slides" ]; then
    missing=true
else
    i=0
    while [ $i -le $LAST ]; do
        if [ ! -f "slides/slide_${{i}}.png" ]; then
            missing=true
            break
        fi
        i=$((i+1))
    done
fi

if [ "$missing" = true ]; then
    echo "Slides missing — running cargo run --bin screencap..."
    cargo run --bin screencap
else
    echo "Slides found in 'slides/' — skipping screencap."
fi

echo "Creating input list..."
rm -f input.txt
"#,
            player = project.player_name,
            total = total,
            last = last
        );

        let mut file = File::create("render.sh")?;
        file.write_all(script_content.as_bytes())?;

        for (i, record) in project.records.iter().enumerate() {
            let line = format!("echo \"file 'slides/slide_{}.png'\" >> input.txt\n", i);
            file.write_all(line.as_bytes())?;
            let line_dur = format!(
                "echo \"duration {}\" >> input.txt\n",
                record.display_duration_sec
            );
            file.write_all(line_dur.as_bytes())?;
        }

        let footer = r#"
echo "file 'slides/slide_0.png'" >> input.txt
echo "Rendering video..."
ffmpeg -f concat -safe 0 -i input.txt -vsync vfr -pix_fmt yuv420p output_b50.mp4
echo "Done!"
"#;
        file.write_all(footer.as_bytes())?;

        // Make the script executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o755);
            file.set_permissions(perms)?;
        }
        Ok(())
    }
}
