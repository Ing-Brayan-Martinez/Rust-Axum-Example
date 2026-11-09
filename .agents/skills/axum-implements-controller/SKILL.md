---
name: axum-implements-controller
description: "Crear o modificar controllers, handlers, rutas REST Axum y respuestas HTTP en este proyecto Rust. Úsala para implementar endpoints CRUD, extractores, códigos de estado, errores y pruebas de rutas."
---

# Implementación de controllers Axum

Usa esta skill al agregar o cambiar endpoints HTTP. En este proyecto, los controllers registran rutas Axum, extraen datos HTTP, llaman a un service y convierten sus resultados en respuestas; no contienen lógica de negocio ni consultas SQL.

## Procedimiento

1. **Revisa los patrones existentes.** Consulta un controller de recurso en `src/controller/` (por ejemplo, `task_controller.rs`), su modelo en `src/domain/`, su service en `src/service/`, `src/controller/mod.rs`, `src/lib.rs` y las pruebas pertinentes en `tests/`. Revisa también `README.md` para mantener el contrato documentado de rutas y JSON.

2. **Define el contrato HTTP antes de codificar.** Determina recurso, método, ruta, parámetros, cuerpo JSON, respuesta y códigos de estado. Mantén el prefijo `/api/v1` y los nombres y métodos existentes salvo que el cambio solicite explícitamente una modificación compatible y documentada.

3. **Registra las rutas en el router del recurso.** Implementa o actualiza `pub fn router() -> Router` en `src/controller/<recurso>_controller.rs`; importa los tipos de `axum::routing` que necesites y encadena los métodos sobre `.route(...)`. Sigue el patrón del proyecto:

   ```rust
   Router::new()
       .route("/api/v1/task", get(find_all).post(create).put(update))
       .route("/api/v1/task/{id}", get(find_by_id).delete(delete))
   ```

   Axum 0.8 usa `{id}` para segmentos dinámicos. Asegúrate de que la firma del handler y el extractor coincidan con el método y la ruta.

4. **Extrae los datos HTTP con extractores tipados.**
   - Usa `Json<T>` para cuerpos JSON y tipos definidos en `domain`.
   - Usa `Path<Uuid>` (o el tipo real del identificador) para parámetros de ruta.
   - No dupliques validación o lógica de negocio que corresponda al service. Los errores de extracción de Axum, como JSON inválido o UUID mal formado, deben conservar el comportamiento HTTP de Axum; no los ocultes con valores predeterminados.

5. **Delega la operación al service.** El handler debe llamar a `service::<recurso>_service::<operación>(...)` y esperar el resultado con `.await`. No accedas a SQLx, al pool ni al repositorio desde el controller. No bloquees el runtime.

6. **Convierte el resultado en una respuesta apropiada.** Sigue los tipos de respuesta existentes:
   - `Result<Json<T>, StatusCode>` para una respuesta JSON exitosa.
   - `(StatusCode, Json<T>)` cuando se requiere un estado explícito, como `201 Created`.
   - `StatusCode::NO_CONTENT` para una operación de borrado exitosa sin cuerpo.
   - Mapea registro inexistente a `404 Not Found` y error de base de datos a `500 Internal Server Error`, usando `ServiceError` y un helper privado local como `service_error`.
   - Registra errores internos con `tracing::error!` sin filtrar API keys, credenciales ni datos sensibles; no conviertas errores en respuestas exitosas ni los descartes silenciosamente.

   Patrón existente para una creación:

   ```rust
   async fn create(Json(input): Json<TaskCreate>) -> Result<(StatusCode, Json<Task>), StatusCode> {
       task_service::create(input)
           .await
           .map(|task| (StatusCode::CREATED, Json(task)))
           .map_err(service_error)
   }
   ```

7. **Conecta el controller a la aplicación.** Declara el módulo en `src/controller/mod.rs`. Si es un recurso nuevo, importa su módulo en `src/lib.rs` y combina su `router()` dentro de `api_routes`, de modo que quede detrás del middleware existente de `X-API-Key`. No muevas la ruta pública `GET /` ni debilites la autenticación de `/api/v1/*`.

8. **Añade o actualiza pruebas.** Usa pruebas de integración con `create_app`, `tower::ServiceExt::oneshot`, `Request` y `StatusCode`, siguiendo `tests/api_routes.rs`. Comprueba como mínimo el método/ruta y los estados relevantes. Ten en cuenta:
   - Las rutas de API requieren `X-API-Key`; incluye una clave de prueba en solicitudes autenticadas.
   - Las pruebas que ejecutan operaciones CRUD necesitan PostgreSQL y el esquema descrito en `README.md`. No hagas que la suite predeterminada dependa de una base de datos que no esté disponible.
   - Incluye casos de error relevantes (por ejemplo, no autorizado, identificador inválido o registro inexistente) sin debilitar la validación real de producción.

9. **Actualiza la documentación y valida.** Actualiza la tabla de endpoints y ejemplos JSON de `README.md` si cambia el contrato. Ejecuta:

   ```bash
   cargo fmt --check
   cargo check
   cargo test
   ```

   Si las pruebas requieren PostgreSQL, deja claro ese requisito y sigue el patrón de la prueba de base de datos ignorada; no informes validaciones que no ejecutaste.

## Reglas del proyecto

- Conserva la dirección `controller → service → repository`.
- Los controllers gestionan HTTP; el service gestiona la lógica de aplicación.
- Mantén respuestas, códigos de estado, rutas y serialización consistentes con el contrato existente.
- No registres cuerpos de solicitud ni credenciales si pueden contener información sensible.
- Usa las versiones y dependencias ya declaradas en `Cargo.toml`; no agregues abstracciones o dependencias sin necesidad.
