mod gpu;

use std::io::{self, Read, Write};
use std::net::TcpListener;
use sysinfo::System;

const BYTES_PER_GIB: f64 = 1024.0 * 1024.0 * 1024.0;
const NOT_FOUND_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\
Connection: close\r\n\r\n";

fn get_os_name() -> String {
	System::long_os_version()
		.unwrap_or_else(|| "Unknown".to_string())
}

fn get_memory(sys: &System) -> String {
	let total_gib = sys.total_memory() as f64 / BYTES_PER_GIB;

	format!(
		"{:.2} GiB",
		total_gib
	)
}

fn get_cpu_name(sys: &System) -> String {
	let model = sys
		.cpus()
		.first()
		.map(|c| c.brand().trim())
		.unwrap_or("Unknown");

	format!(
		"{} ({})",
		model,
		sys.cpus().len()
	)
}

fn create_static_response() -> Vec<u8> {
	let mut sys = System::new();
	sys.refresh_cpu_all();
	sys.refresh_memory();

	let hostname = System::host_name()
		.unwrap_or_else(|| "Unknown".to_string());
	let os = get_os_name();
	let cpu = get_cpu_name(&sys);
	let gpu = gpu::get_name();
	let memory = get_memory(&sys);

	let response = format!(
		"HTTP/1.1 200 OK\r\n\
		Connection: close\r\n\
		Cache-Control: max-age=31536000\r\n\
		Cloudflare-CDN-Cache-Control: max-age=31536000\r\n\
		Access-Control-Allow-Origin: https://klartnet.io\r\n\
		Content-Type: application/json\r\n\r\n\
		{}",
		format!(
			r#"{{"hostname": "{}", "os": "{}", "cpu": "{}", "gpu": "{}", "memory": "{}"}}"#,
			hostname, os, cpu, gpu, memory
		)
	);

	response.into_bytes()
}

fn main() -> io::Result<()> {
	let static_response = create_static_response();


	let listener = TcpListener::bind("[::]:9100")?;

	println!("Server started: http://localhost:9100/");

	let mut discard_buffer = [0u8; 4096];

	for mut stream in listener.incoming().flatten() {
		if let Ok(n) = stream.read(&mut discard_buffer) {
			if discard_buffer[..n].starts_with(b"GET /spec.csv ") {
				let _ = stream.write_all(&static_response);
			} else {
				let _ = stream.write_all(NOT_FOUND_RESPONSE);
			}
		}
	}

	Ok(())
}