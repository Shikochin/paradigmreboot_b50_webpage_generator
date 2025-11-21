use crate::models::B50Project;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct ScriptGenerator;

impl ScriptGenerator {
    pub fn generate(project: &B50Project) -> std::io::Result<()> {
        let script_content = format!(
            r#"
# Paradigm B50 Render Script
# Generated for {}
# Note: Since the UI is now a Single Page App, ensure you capture screenshots
# matching the timing or use a screen recorder.
echo "Creating input list..."
rm -f input.txt
"#,
            project.player_name
        );

        let mut file = File::create("render.sh")?;
        file.write_all(script_content.as_bytes())?;

        for (i, record) in project.records.iter().enumerate() {
            let line = format!("echo \"file 'slide_{}.png'\" >> input.txt\n", i);
            file.write_all(line.as_bytes())?;
            let line_dur = format!(
                "echo \"duration {}\" >> input.txt\n",
                record.display_duration_sec
            );
            file.write_all(line_dur.as_bytes())?;
        }

        let footer = r#"
echo "file 'slide_0.png'" >> input.txt
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
