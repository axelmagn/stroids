#!/bin/bash
set -exuo pipefail

readonly BASE_DIR="$(cd $(dirname $(realpath $0))/.. && pwd)"

pushd "${BASE_DIR}"
	wasm-pack build --target web --dev
	python3 -m http.server
popd