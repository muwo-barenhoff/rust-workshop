# Rust Workshop

## Installation

Eine Installationsanleitung findet ihr [hier](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) im rust book.

* [Microsoft Build Tools 2015 Update 3](https://visualstudio.microsoft.com/de/vs/older-downloads/)
* [Rust Toolchain](https://www.rust-lang.org/tools/install)
  * Rust Analyzer (language server): `rustup.exe component add rust-analyzer`
  * Clippy (linter):  `rustup.exe component add clippy`
  * rust-fmt (Source Code formatter): `rustup.exe component add rustfmt`
  * rust-docs (Bibliotheks Referenz Doku): `rustup.exe component add rust-docs`
* [Visual Studio Code](https://code.visualstudio.com/Download)
  * rust-analyzer Extension
  * Cargo Extension
  * Even Better TOML
  * CodeLLDB

## Ein paar Hintergründe

* 2009 vom Mozilla Mitarbeiter Graydon Hoare entwickelt
* Erster rustc compiler in rust 2011, vorher ocaml implementation
* Mittlerweile 2021 Rust Foundation (u.a. Mozilla, AWS, Google et.al)
* Mittlerweile sowohl auf Desktop, Server als auch Embedded eingesetzt
* Bekannte Projekte die rust verwenden:
  * Mozilla Firefox
  * [Redox OS](https://www.redox-os.org/) (Betriebssystem)
  * [Linux Kernel Module in rust schreiben](https://github.com/Rust-for-Linux) (seit Linux 6.1 im offiziellem Kernel)
  * [Signal Messanger](https://github.com/signalapp?language=rust)
  * [Google fuchsia kernel](https://fuchsia.dev/fuchsia-src/development/languages/rust)
* [crates.io](https://crates.io/) Bibliotheks repository mit mittlerweile > 130.000 Bibliotheken
* Breites Einsatzspektrum
  * Kommandozeilen und Desktop Anwendungen
  * Web und Serverbackend Anwendungen
  * Embedded (bare-metal) Microcontroller Entwicklung
* Striktes Typsystem (jedes Sprachobjekt hat bekannten Typ zur Compiletime)
* "Ownership" Memory Managment (weder malloc()/free() noch Garbage Collector nötig)
* Memory Safety by design (keine Pointer Arithmetik, keine Null Pointer Exceptions etc)
* "unsicherer" Code trotzdem möglich (unsafe Blöcke) - Hohe flexibilität

## Link Sammlung

* ["The Rust Programming Language"](https://doc.rust-lang.org/stable/book/title-page.html) - Gute Einführung für Anfänger
* ["Rust by Example"](https://doc.rust-lang.org/stable/rust-by-example/) - Einführung an klassischen Beispielen
* ["The Rust Reference"](https://doc.rust-lang.org/stable/reference/) - Referenz Handbuch zur Sprache
* [RustDoc std](https://doc.rust-lang.org/std/) - Referenz zur Standard Bibliothek
* [crates.io](https://crates.io/) - Bibliotheks Repository
* ["The Embedded Rust Book"](https://docs.rust-embedded.org/book/) - Rust auf dem Mikrocontroller

## Sessions

### Rust Hands on Workshop 1

Im ersten Teil des Workshops haben wir die Rust Entwicklungsumgebung installiert und es
wurden die Basis Datentypen sowie Flow Control Strukturen behandelt.

Eine Aufzeichnung der Session findet sich hier:

* [Rust HandsOn Training 1](https://multivaccloud-my.sharepoint.com/personal/markus_barenhoff_multivac_de/_layouts/15/stream.aspx?id=%2Fpersonal%2Fmarkus%5Fbarenhoff%5Fmultivac%5Fde%2FDocuments%2FAufnahmen%2FRust%20HandsOn%20Training%2Emp4&referrer=StreamWebApp%2EWeb&referrerScenario=AddressBarCopied%2Eview&ga=1)

Der Rust Source Code findet sich unter: [src/base.rs](./src/base.rs).
