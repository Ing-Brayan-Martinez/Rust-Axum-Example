---
name: axum-implements-service
description: "Crear o modificar services de aplicación en esta API Rust/Axum. Úsala para implementar lógica de negocio, validaciones, operaciones CRUD, coordinación con repositories y manejo de errores."
---

# Implementación de services

Usa esta skill al agregar o cambiar lógica de aplicación. En este proyecto, los services son funciones asíncronas en `src/service/`: reciben tipos del dominio, coordinan operaciones del repository y devuelven `Result<..., ServiceError>`. Los controllers traducen sus resultados a HTTP; los repositories ejecutan SQLx.

## Procedimiento paso a paso

1. **Inspecciona el flujo y los tipos existentes.** Revisa el service más parecido en `src/service/`, el controller que lo invoca, el repository relacionado, los tipos de `src/domain/` y `src/config/helpers.rs`. Confirma la semántica del endpoint, las columnas del esquema y los códigos HTTP antes de implementar.

2. **Mantén la separación de responsabilidades.**
   - El service implementa lógica de aplicación, transforma DTOs de entrada en modelos y coordina operaciones.
   - El repository es responsable de consultas y persistencia.
   - El controller extrae datos HTTP y mapea `ServiceError` a respuestas.
   No ejecutes SQL ni devuelvas `StatusCode` desde el service.

3. **Define firmas tipadas y asíncronas.** Sigue el patrón existente de funciones `pub async fn` con argumentos de dominio y retorno `Result<T, ServiceError>`. Usa `async`/`await` y propaga errores con `?` cuando `From<sqlx::Error> for ServiceError` sea suficiente; usa `map_err(ServiceError::Database)` cuando haga más explícita la conversión.

4. **Implementa las operaciones con resultados explícitos.**
   - En `create`, construye el modelo con `Uuid::new_v4()` y, cuando corresponda, fechas UTC como `Utc::now().naive_utc()`, después llama al repository y devuelve el registro persistido.
   - En `find_all`, propaga el resultado del repository.
   - En `find_by_id`, transforma `Option<Model>` en `ServiceError::NotFound` cuando el contrato requiera que el registro exista.
   - En `update`, no uses `unwrap()` sobre el resultado de `find_by_id` o `update`. Convierte cada `None` en `ServiceError::NotFound`; modifica los campos permitidos y preserva los campos que no deben cambiar, como el identificador y `created_at`.
   - En `delete`, interpreta `false` como `NotFound` y `true` como éxito (`()`).

5. **Ubica validación y errores correctamente.** Aplica en el service las reglas de negocio y validaciones que deben cumplirse independientemente del transporte HTTP; deja a Axum los errores de sintaxis/extracción HTTP y al repository los errores de SQL. Propaga fallos de base de datos como `ServiceError::Database`, sin convertirlos en `NotFound` ni en éxito. Si agregas una nueva clase de error, actualiza `ServiceError` y todos los controladores que la mapean.

6. **Mantén trazas seguras y útiles.** Usa `#[tracing::instrument]` para operaciones si aporta contexto y `skip(input)` para argumentos que puedan contener datos sensibles. No registres API keys, credenciales, cuerpos completos, ni valores personales o comerciales innecesarios. No clones un modelo únicamente para imprimirlo; evita logs de payloads y usa mensajes estructurados sin secretos.

7. **Añade pruebas al nivel adecuado.** Prueba reglas de negocio y transformaciones sin requerir PostgreSQL cuando puedan aislarse en funciones pequeñas. Para comportamiento que depende del repository, usa pruebas de integración con una base de datos aislada; la conexión está centralizada en `src/config/postgresql_conmection.rs` y las pruebas de PostgreSQL deben quedar fuera de la suite predeterminada si necesitan servicios externos. No agregues dependencias de mocking sin necesidad concreta ni hagas que pruebas concurrentes compartan datos mutables sin aislamiento.

8. **Actualiza las capas dependientes.** Si cambia una firma o semántica del service, actualiza el controller correspondiente, sus pruebas, los tipos de dominio si hace falta y `README.md` cuando cambie el contrato HTTP. Conserva el flujo `controller → service → repository`.

9. **Valida el cambio.** Ejecuta los chequeos adecuados:

   ```bash
   cargo fmt --check
   cargo check
   cargo test
   ```

   Ejecuta pruebas con PostgreSQL explícitamente solo cuando estén configuradas y el cambio lo requiera. Informa con precisión qué comandos se ejecutaron.

## Reglas del proyecto

- Usa `ServiceError` (`NotFound` y `Database`) como resultado de aplicación, ampliándolo de forma coherente si el requisito lo exige.
- Conserva UUIDs y tipos de dominio existentes; sincroniza las operaciones con el esquema PostgreSQL documentado.
- Usa `Result` en vez de `unwrap`, `expect` o errores descartados para operaciones que puedan fallar.
- No introduzcas runtime reactivo adicional: el proyecto usa Tokio y funciones `async`.
- Evita abstraer una sola operación en capas adicionales sin una necesidad clara.
