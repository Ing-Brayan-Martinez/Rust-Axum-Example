# Guía para agentes y desarrolladores

## Contexto del proyecto

API REST de ejemplo construida con Rust 2024, Axum, Tokio y SQLx, con PostgreSQL como base de datos. Consulta [README.md](README.md) para los requisitos de instalación, el esquema SQL y los endpoints.

## Estructura y arquitectura

- `src/main.rs`: carga la configuración, inicializa el logging y arranca el servidor.
- `src/lib.rs`: expone los módulos y compone la aplicación y sus rutas.
- `src/config/`: configuración, autenticación por API key y conexión a PostgreSQL.
- `src/controller/`: rutas Axum, extracción de datos HTTP y conversión de errores a respuestas HTTP.
- `src/service/`: lógica de aplicación y coordinación de operaciones asíncronas.
- `src/repository/`: acceso a PostgreSQL mediante SQLx y mapeo de filas a modelos.
- `src/domain/`: modelos y estructuras de entrada/salida.
- `tests/`: pruebas de integración de autenticación, rutas y base de datos.

Mantén el flujo `controller → service → repository`. Los controladores no deben ejecutar consultas SQL ni contener lógica de persistencia; los repositorios no deben decidir respuestas HTTP.

## Pautas de implementación

- Sigue Rust 2024 y los patrones existentes: funciones asíncronas, `Result` para errores y tipos explícitos en las fronteras entre capas.
- Usa `async`/`await` de Tokio para operaciones de I/O; no bloquees el runtime. Mantén el estilo asíncrono existente en lugar de introducir abstracciones reactivas nuevas sin necesidad concreta.
- Usa UUID para los identificadores, como en los modelos y servicios actuales.
- Para consultas SQL con datos variables, usa parámetros enlazados de SQLx (`.bind(...)`); no construyas SQL concatenando entradas.
- Mantén el mapeo de errores explícito: los registros inexistentes se traducen a `404` y los errores de base de datos a `500`, siguiendo `ServiceError` y los controladores existentes.
- Preserva el contrato HTTP y JSON existente. Si lo cambias, actualiza o añade pruebas y ajusta la documentación pertinente en `README.md`.
- Usa `tracing` para diagnóstico; no registres API keys, credenciales ni valores sensibles de entrada.

## Configuración y seguridad

- `API_KEY` es obligatoria y no puede estar vacía; la aplicación debe fallar al iniciar si falta.
- Las rutas `/api/v1/*` requieren el encabezado `X-API-Key`; `GET /` es público.
- La comparación de API keys debe seguir siendo de tiempo constante usando el mecanismo existente.
- `DATABASE_URL` configura PostgreSQL y `PORT` configura el servidor (por defecto, `3000`). La conexión a la base de datos se inicializa al necesitarla.
- Usa `.env.example` como referencia. No agregues secretos a archivos versionados ni registres `.env`.
- El proyecto no aplica migraciones automáticamente. Si un cambio requiere modificar el esquema, documenta el SQL necesario en `README.md`.

## Validación

Ejecuta los chequeos adecuados al cambio. Para validar el proyecto:

```bash
cargo fmt --check
cargo check
cargo test
```

La prueba de PostgreSQL está ignorada por defecto. Con una base de datos disponible y `DATABASE_URL` configurada, ejecútala explícitamente:

```bash
cargo test --test database -- --ignored
```

Para iniciar la API localmente, configura `API_KEY` y las demás variables necesarias, y ejecuta `cargo run`. No afirmes que una validación pasó si no la ejecutaste.
