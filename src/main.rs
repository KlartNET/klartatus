mod gpu;

use std::io::{Read, Write};
use std::net::{TcpListener};
use sysinfo::System;

const BYTES_PER_GIB: f64 = 1024.0 * 1024.0 * 1024.0;

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
		Access-Control-Allow-Origin: *\r\n\
		Content-Type: application/json\r\n\
		Connection: Close\r\n\r\n\
		{{\"hostname\": \"{}\", \"os\": \"{}\", \"cpu\": \"{}\", \"gpu\": \"{}\", \"memory\": \"{}\", \"uptime\": ",
		hostname, os, cpu, gpu, memory
	);

	response.into_bytes()
}

fn main() -> std::io::Result<()> {
	let static_part = create_static_response();
	let listener = TcpListener::bind("[::]:9100")?;

	println!("서버 시작됨: http://localhost:9100/");

	for mut stream in listener.incoming().flatten() {
		let mut discard_buffer = [0; 4096];
		let _ = stream.read(&mut discard_buffer);

		let _ = stream.write_all(&static_part);

		let _ = write!(stream, "{}}}", System::uptime());
	}

	Ok(())
}