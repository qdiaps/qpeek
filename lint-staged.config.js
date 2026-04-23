export default {
  '*.{js,ts,vue}': ['eslint --fix', 'prettier --write'],
  '*.rs': () => [
    'cargo fmt --manifest-path src-tauri/Cargo.toml',
    'cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings'
  ],
  '*.{json,md,yml}': ['prettier --write']
}
