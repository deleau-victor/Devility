fn main() {
	tauri_build::build();
	#[cfg(windows)]
	embed_resource::compile("windows/admin-manifest.rc", embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
