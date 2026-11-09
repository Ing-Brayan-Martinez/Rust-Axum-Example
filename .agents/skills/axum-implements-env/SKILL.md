---
name: axum-implements-env
description: "Implementar o modificar variables de entorno en esta API Rust con Axum: carga de .env, configuración de PORT, DATABASE_URL y API_KEY, validación, pruebas y documentación."
---

# Implementación de variables de entorno

Usa esta skill cuando agregues o cambies configuración por variables de entorno en este proyecto. Sigue el patrón existente con Rust 2024, `dotenvy`, Tokio, Axum y SQLx; evita introducir un sistema de configuración distinto sin un requisito concreto.

## Variables existentes

| Variable | Requisito y comportamiento | Ubicación actual |
| --- | --- | --- |
| `API_KEY` | Obligatoria y no vacía. Se valida al arrancar; protege todas las rutas `/api/v1/*` mediante `X-API-Key`. | `src/config/api_key.rs`, `src/main.rs`, `src/lib.rs` |
| `PORT` | Opcional; usa `3000` si falta. Debe parsear como `u16`; un valor inválido hace fallar el arranque. | `src/main.rs` |
| `DATABASE_URL` | Necesaria para conectar a PostgreSQL; se lee al inicializar la conexión, no durante el arranque del servidor. | `src/config/postgresql_conmection.rs` |

`src/main.rs` carga `.env` con `dotenvy` antes de leer la API key. La ausencia del archivo `.env` es válida; otros errores al leerlo se propagan. Las variables del proceso también pueden configurarse desde el entorno de ejecución.

## Procedimiento

1. **Inspecciona el patrón y los usos actuales.** Revisa `src/main.rs`, `src/config/`, `Cargo.toml`, `.env.example`, `.gitignore`, las pruebas afectadas y la sección de configuración de `README.md`. Busca todos los usos de la variable para evitar configuraciones inconsistentes.

2. **Define el contrato de configuración.** Decide si la variable es obligatoria, su valor predeterminado (si corresponde), el tipo y rango válidos, cuándo se lee y qué error debe producir. Mantén mensajes claros y no incluyas valores secretos en ellos.

3. **Lee y valida en el punto adecuado.**
   - Para configuración necesaria antes de iniciar el servidor, lee después de cargar `.env` en `main` o en una función de configuración usada desde allí.
   - Para configuración propia de una integración, sigue el patrón de esa integración; por ejemplo, `DATABASE_URL` se lee en `get_db()` y el pool se inicializa de forma diferida.
   - Propaga errores con `Result`; no sustituyas variables ausentes o inválidas por valores que oculten el error. Conserva el valor predeterminado de `PORT` solo cuando la variable no está definida; un puerto definido pero inválido debe producir error.
   - Mantén `API_KEY` obligatoria y no vacía. No relajes su validación ni su protección de rutas al cambiar configuración relacionada.

4. **Actualiza el ejemplo y la documentación.** Añade o ajusta la variable en `.env.example` usando valores de ejemplo claramente no secretos. Actualiza `README.md` con su propósito, si es obligatoria, su valor predeterminado, cómo configurarla localmente y el efecto de un valor inválido. No copies credenciales reales a estos archivos.

5. **Añade pruebas de comportamiento.** Prueba valores válidos, ausencia, valor vacío y formato o rango inválidos según el contrato. Aísla el acceso al entorno cuando sea necesario para que las pruebas no dependan del `.env` o de variables locales. No uses secretos reales. Si las pruebas existentes no permiten controlar la configuración de proceso de manera segura, extrae una función pequeña que acepte el valor como entrada y prueba esa función.

6. **Verifica el cambio.** Ejecuta `cargo fmt --check`, `cargo check` y las pruebas pertinentes; normalmente, `cargo test`. La prueba `tests/database.rs` está ignorada por defecto y requiere PostgreSQL y `DATABASE_URL`; ejecútala solo si el cambio la afecta y la base de datos está disponible:

   ```bash
   cargo test --test database -- --ignored
   ```

7. **Revisa seguridad y consistencia.** Confirma que `.env` no se agrega al control de versiones, que los logs no muestran claves, credenciales ni URLs con secretos y que el valor se lee de forma consistente en todos sus consumidores.

## Restricciones del proyecto

- No registres API keys ni credenciales con `tracing` o `log`.
- No pongas secretos en `.env.example`, pruebas, mensajes de error ni documentación versionada.
- No leas `DATABASE_URL` en controladores o servicios; el acceso a la base de datos pertenece a `src/config/` y `src/repository/`.
- No cambies silenciosamente la autenticación: `GET /` es público y las rutas `/api/v1/*` requieren `X-API-Key`.
- Este proyecto no realiza migraciones automáticas. Si una nueva variable exige cambios de esquema, documenta el SQL requerido en `README.md`.
