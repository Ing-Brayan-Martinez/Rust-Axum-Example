---
name: axum-implements-test
description: "Crear, actualizar y ejecutar pruebas para esta API Rust/Axum. Úsala para tests unitarios, pruebas HTTP con Tower, autenticación, validación de JSON y pruebas de integración con PostgreSQL."
---

# Pruebas para Rust y Axum

Usa esta skill al probar cambios en handlers, services, repositories, configuración o contratos HTTP. El proyecto usa pruebas de integración en `tests/`, Tokio para asincronía y Tower para invocar routers Axum sin levantar un servidor.

## Procedimiento paso a paso

1. **Define qué comportamiento debe verificarse.** Revisa el código afectado, sus consumidores y las pruebas existentes. Escribe casos observables para éxito, errores y límites; enfócate en el contrato y evita probar detalles de implementación que puedan cambiar sin alterar el comportamiento.

2. **Elige el nivel de prueba apropiado.**
   - Prueba lógica pura y transformaciones pequeñas sin red ni base de datos.
   - Usa pruebas HTTP de integración con `create_app`, `tower::ServiceExt::oneshot` y `Request` para rutas, middleware, códigos de estado y respuestas.
   - Usa PostgreSQL solo para comportamiento que dependa realmente de SQL, esquema, tipos o persistencia.
   No añadas librerías de mocking o infraestructura de pruebas sin una necesidad clara.

3. **Escribe pruebas HTTP siguiendo el patrón del repositorio.** Crea o actualiza archivos bajo `tests/`, importa `Body`, `Request`, `StatusCode`, `create_app` y `tower::ServiceExt` según lo necesario. Crea un router por prueba con una API key ficticia:

   ```rust
   let app = create_app("test-key".into());
   let response = app
       .oneshot(
           Request::builder()
               .uri("/")
               .body(Body::empty())
               .unwrap(),
       )
       .await
       .unwrap();

   assert_eq!(response.status(), StatusCode::OK);
   ```

   `oneshot` consume el router; construye una nueva instancia para cada solicitud. Mantén nombres descriptivos en formato `snake_case` y aserciones específicas.

4. **Cubre seguridad y límites HTTP.** Las rutas `/api/v1/*` requieren `X-API-Key`, mientras `GET /` es público. Prueba clave correcta, ausente e incorrecta en los cambios de autenticación. Para handlers, cubre métodos y rutas, JSON válido e inválido, parámetros de ruta, registros inexistentes y códigos HTTP relevantes. No incluyas claves reales en pruebas ni en logs.

5. **Verifica cuerpos y datos, no solo el estado.** Cuando el contrato dependa de la respuesta, lee el body con las utilidades de Axum y verifica el JSON y los campos significativos. Si el body debe estar vacío (por ejemplo, `204 No Content`), comprueba también que no se devuelvan datos. Evita aserciones frágiles sobre orden de campos JSON.

6. **Prueba errores sin ocultar fallos.** Las pruebas pueden usar `unwrap()` en la construcción de requests o respuestas cuando un fallo representa un error de programación del test; el código de producción debe seguir propagando errores explícitamente. No conviertas una conexión fallida, un parseo fallido o una aserción ausente en una prueba exitosa.

7. **Aísla pruebas que requieren PostgreSQL.**
   - La prueba `tests/database.rs` está marcada `#[ignore]` porque requiere PostgreSQL y `DATABASE_URL`.
   - Mantén `cargo test` normal ejecutable sin una instancia de base de datos.
   - Ejecuta una prueba de integración de base de datos solo con una base de pruebas dedicada y el esquema esperado por `README.md`.
   - Usa datos aislados y limpia los registros creados; evita conflictos entre pruebas concurrentes y nunca apuntes accidentalmente a una base de producción.

8. **Ejecuta la validación.** Comienza con el archivo de prueba afectado cuando el runner lo permita; para validar el proyecto usa:

   ```bash
   cargo fmt --check
   cargo check
   cargo test
   ```

   Para ejecutar explícitamente el smoke test de PostgreSQL, solo con una base de pruebas disponible:

   ```bash
   cargo test --test database -- --ignored
   ```

   La CI actual ejecuta `cargo build --verbose` y `cargo test --verbose`. Ejecuta esos comandos si el cambio afecta build o configuración de CI.

9. **Actualiza la cobertura del contrato.** Si el cambio modifica una ruta, un campo JSON, un código de estado o un requisito de configuración, actualiza las pruebas y la documentación pertinente en `README.md`. Informa qué comandos se ejecutaron y no afirmes que una prueba pasó si no se ejecutó.

## Reglas del proyecto

- Usa Tokio y Tower ya presentes en `Cargo.toml`; no levantes un servidor real para probar routers que pueden verificarse con `oneshot`.
- No hagas que pruebas HTTP simples dependan de `DATABASE_URL` o PostgreSQL.
- Conserva la distinción entre `GET /` público y endpoints `/api/v1/*` autenticados.
- No compartas secretos ni datos reales en fixtures, mensajes de fallo o logs.
- Si una prueba falla, identifica si es regresión, entorno externo o estado previo antes de cambiar producción.
