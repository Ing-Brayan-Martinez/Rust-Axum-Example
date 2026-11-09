---
name: axum-implements-lint
description: "Revisar formato, compilación, advertencias y pruebas de este proyecto Rust/Axum. Úsala para chequear cambios, investigar fallos de lint o configurar validaciones de calidad con Cargo."
---

# Validación y lint de Rust

Usa esta skill para comprobar cambios de código en el proyecto Rust 2024. El repositorio no configura Checkstyle, Maven, `rustfmt.toml` ni reglas propias de Clippy; CI ejecuta `cargo build --verbose` y `cargo test --verbose`.

## Procedimiento

1. **Inspecciona la configuración y el alcance del cambio.** Revisa `Cargo.toml`, `.github/workflows/rust.yml`, `AGENTS.md` y los archivos modificados. Comprueba si hay configuración de `rustfmt` o Clippy antes de proponer cambios; no asumas reglas que no existen.

2. **Ejecuta primero formato en modo de verificación.**

   ```bash
   cargo fmt --check
   ```

   Si falla por formato, aplica `cargo fmt` y revisa el diff para asegurarte de que solo sean cambios de formato esperados. No uses `cargo fmt` para ocultar cambios funcionales no relacionados.

3. **Comprueba que el proyecto compile.**

   ```bash
   cargo check
   ```

   Para una validación de build que refleje CI, ejecuta también `cargo build --verbose`. Si falla, corrige el error raíz y vuelve a correr las verificaciones relevantes.

4. **Ejecuta Clippy como análisis estático complementario.**

   ```bash
   cargo clippy --all-targets --all-features
   ```

   Revisa cada advertencia y corrige las que sean pertinentes al cambio. No añadas `-D warnings` ni suprimas lints globalmente salvo que el usuario solicite establecer esa política y se haya comprobado que es compatible con el código existente. Para excepciones puntuales, prefiere una justificación localizada en vez de silenciar una categoría amplia.

5. **Ejecuta las pruebas.**

   ```bash
   cargo test
   ```

   La suite normal no debe requerir una base PostgreSQL disponible. La prueba `tests/database.rs` está ignorada por defecto; si el cambio afecta esa integración y PostgreSQL está configurado, ejecútala explícitamente:

   ```bash
   cargo test --test database -- --ignored
   ```

6. **Si el cambio modifica CI o políticas de lint, actualiza CI de forma consistente.** El workflow actual está en `.github/workflows/rust.yml` y ejecuta build y tests. Añade formato o Clippy al workflow solo si el alcance pide hacerlas verificaciones obligatorias; actualiza documentación relacionada y valida los comandos localmente.

7. **Informa los resultados con precisión.** Indica qué comandos se ejecutaron y cuáles pasaron o fallaron. Distingue los chequeos locales de los del workflow; no afirmes que CI pasó si no fue ejecutado. Si un comando no pudo correrse por una herramienta ausente o por requerir PostgreSQL, explica la limitación.

## Reglas del proyecto

- Usa Cargo y las herramientas del toolchain Rust del proyecto; no introduzcas Maven, Checkstyle u otros linters de otro ecosistema.
- No cambies código ajeno al problema solo para silenciar advertencias preexistentes.
- No instales dependencias para ejecutar los chequeos normales del proyecto; las herramientas estándar son `cargo fmt`, `cargo check`, `cargo clippy` y `cargo test`.
- Mantén secretos y configuración local fuera de logs y artefactos versionados.
