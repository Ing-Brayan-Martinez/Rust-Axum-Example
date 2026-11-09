---
name: axum-implements-openapi
description: "Agregar o mantener documentación OpenAPI/Swagger en esta API Rust con Axum: describir rutas, esquemas, autenticación X-API-Key, JSON y códigos de respuesta paso a paso."
---

# Implementación de OpenAPI

Usa esta skill cuando agregues o actualices la especificación OpenAPI, Swagger UI o la documentación de endpoints. Este proyecto usa Axum 0.8 y Serde, pero aún no incluye OpenAPI ni Swagger; revisa las rutas y modelos reales antes de documentarlos.

## Procedimiento paso a paso

1. **Inspecciona la API real.** Revisa `src/lib.rs`, `src/controller/`, `src/domain/`, `src/config/api_key.rs`, `tests/api_routes.rs` y la tabla de endpoints de `README.md`. Confirma rutas, métodos, extractores, cuerpos, nombres JSON, estados HTTP, errores y requisitos de autenticación. No derives la especificación solo del README: comprueba que coincida con el código.

2. **Elige una integración compatible.** Para generación OpenAPI integrada con Axum, evalúa `utoipa` y su integración de Swagger UI para Axum. Antes de añadir dependencias, comprueba versiones compatibles con Axum 0.8, el toolchain/MSRV y el API vigente de los crates. No incorpores OpenAPI manual y generado a la vez como fuentes duplicadas sin una razón clara.

3. **Añade dependencias y verifica la configuración.** Incluye únicamente los crates/features requeridos para generar OpenAPI y, si se solicita, servir Swagger UI. Actualiza `Cargo.lock` mediante Cargo y conserva las versiones compatibles. Mantén la documentación separada del arranque de la API si Swagger UI no forma parte del alcance.

4. **Describe los modelos de dominio y DTOs.** Añade los derives o anotaciones requeridos por la biblioteca elegida a los modelos que aparecen en solicitudes y respuestas. Asegúrate de describir correctamente:
   - UUIDs, decimales y fechas según sus formatos JSON reales.
   - Campos requeridos y tipos de cada campo.
   - Convención de nombres Serde actual; no declares `camelCase` si el código serializa `snake_case`.
   - Diferencias entre estructuras de entrada, actualización y respuesta; no publiques campos que no forman parte del contrato.

5. **Documenta cada operación HTTP.** Añade metadatos de OpenAPI a cada handler/ruta: método y path exactos, resumen, parámetros de ruta, cuerpo JSON, respuestas exitosas y posibles errores. Incluye los estados observados en el código (por ejemplo, `201`, `200`, `204`, `401`, `404` y `500` cuando correspondan); no declares respuestas que la implementación no produce. Para UUID en path, describe el formato y el comportamiento cuando no es válido.

6. **Declara seguridad por API key correctamente.** Define un security scheme de tipo `apiKey` en el encabezado `X-API-Key`. Aplícalo a las operaciones bajo `/api/v1/*`, que están protegidas por middleware, pero no a `GET /`, que es público. No incluyas una clave real ni un valor de ejemplo que parezca una credencial válida en la especificación.

7. **Registra y sirve la especificación.** Integra el documento generado en el router de aplicación de `src/lib.rs` o en el punto de composición que corresponda. Si se agrega Swagger UI, configura explícitamente su URL de especificación y verifica que las rutas de documentación queden fuera del middleware de API key si deben ser públicas. No cambies accidentalmente la protección de los endpoints de negocio ni el endpoint público existente.

8. **Añade pruebas de contrato.** Comprueba que la ruta de especificación responde correctamente, que su JSON puede parsearse como OpenAPI y que contiene rutas, métodos, esquemas y seguridad esperados. Si se habilita Swagger UI, verifica que su ruta se sirve. Mantén las pruebas CRUD que requieren PostgreSQL fuera de la suite predeterminada si no hay una base de datos configurada.

9. **Actualiza documentación y valida.** Documenta en `README.md` dónde obtener el JSON OpenAPI y dónde abrir Swagger UI, si existe. Ejecuta:

   ```bash
   cargo fmt --check
   cargo check
   cargo test
   ```

   Ejecuta también `cargo clippy --all-targets --all-features` si el cambio incluye código y esa herramienta está disponible. Si cambia un contrato HTTP, actualiza tanto OpenAPI como la tabla y los ejemplos del README. No declares la validación exitosa si no corriste los comandos.

## Reglas del proyecto

- OpenAPI debe reflejar la implementación real; no cambies nombres de campos, rutas o estados solo para ajustarlos a la especificación.
- Mantén la dirección `controller → service → repository`; la documentación no debe mover lógica de negocio al controller.
- Preserva `X-API-Key` en `/api/v1/*` y el acceso público a `GET /`.
- No documentes secretos, URLs con credenciales ni datos de producción.
- Evita duplicar manualmente definiciones de rutas o esquemas que puedan derivarse del código.
