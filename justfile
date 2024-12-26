default:
    just --list

# Erstellen und Ausführen
build-and-run:
	cargo build
	cargo run

# Testen und bereinigen
test-and-clean:
	cargo test
	cargo clean

## Formatieren, Prüfen und Empfehlungen
format-check-and-lint:
	cargo fmt
	cargo     check
	cargo     clippy

## Vollständiges erstellen
full-build:
	cargo     build
	cargo     test
	cargo     run

## Release
release:
	just test-and-clean
	cargo       build
	cargo       run