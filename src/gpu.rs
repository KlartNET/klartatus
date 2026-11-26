use std::process::Command;

#[cfg(target_os = "windows")]
pub fn get_name() -> String {
	let output = Command::new("reg")
		.args([
			"query",
			r"HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}",
			"/s",
			"/v",
			"DriverDesc",
		])
		.output();

	if let Ok(out) = output {
		for line in String::from_utf8_lossy(&out.stdout).lines() {
			if let Some((_, val)) = line.split_once("REG_SZ") {
				let name = val.trim();

				if !name.to_lowercase().contains("virtual") {
					return name.to_string();
				}
			}
		}
	}

	"Unknown".to_string()
}

#[cfg(target_os = "linux")]
pub fn get_name() -> String {
	let output = Command::new("lspci").output();

	if let Ok(out) = output {
		for line in String::from_utf8_lossy(&out.stdout).lines() {
			let lower = line.to_lowercase();
			
			if lower.contains("vga") {
				if let Some(val) = line.split(':').nth(2) {
					let name = val.trim();

					if !name.is_empty() {
						return name.to_string();
					}
				}
			}
		}
	}

	"Unknown".to_string()
}

#[cfg(target_os = "macos")]
pub fn get_name() -> String {
	let output = Command::new("system_profiler")
		.arg("SPDisplaysDataType")
		.output();

	if let Ok(out) = output {
		for line in String::from_utf8_lossy(&out.stdout).lines() {
			if let Some((_, val)) = line.split_once("Chipset Model:") {
				let name = val.trim();

				if !name.is_empty() {
					return name.to_string();
				}
			}
		}
	}

	"Unknown".to_string()
}