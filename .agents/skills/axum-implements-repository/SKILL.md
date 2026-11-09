---
name: axum-implements-repository
description: "Crear o modificar repositorios Rust con SQLx y PostgreSQL en esta API Axum. Úsala para implementar consultas CRUD, consultas parametrizadas, mapeo de filas y pruebas de persistencia."
---

# Implementación de repositorios SQLx

Usa esta skill al agregar o cambiar acceso a datos. En este proyecto, los repositorios son módulos asíncronos en `src/repository/`, usan SQLx con PostgreSQL y devuelven errores de SQLx al service. No uses `ReactiveCrudRepository` ni otros patrones de frameworks distintos a Rust.

## Procedimiento paso a paso

1. **Inspecciona la arquitectura y el esquema vigente.** Revisa el repository más parecido (`task_repository.rs`, `product_repository.rs` u `order_repository.rs`), el service que lo consume, el modelo en `src/domain/`, `src/config/postgresql_conmection.rs` y el SQL documentado en `README.md`. Confirma tablas, columnas, tipos y restricciones antes de escribir consultas.

2. **Define operaciones y contratos tipados.** Mantén los repositories como funciones `pub async fn` que retornan `Result<T, sqlx::Error>`. Usa los tipos del dominio y modela explícitamente la ausencia:
   - `Result<T, sqlx::Error>` cuando la operación debe producir una fila.
   - `Result<Option<T>, sqlx::Error>` para búsquedas o actualizaciones donde la fila puede no existir.
   - `Result<bool, sqlx::Error>` para borrados donde `rows_affected()` indica si se eliminó una fila.
   El service traduce estos resultados a errores de aplicación; el repository no debe importar ni decidir códigos HTTP.

3. **Declara e integra el módulo.** Crea `src/repository/<recurso>_repository.rs` y declara `pub mod <recurso>_repository;` en `src/repository/mod.rs`. Conserva la dirección `controller → service → repository`; el controller no debe llamar directamente al repository.

4. **Obtén el pool mediante la configuración existente.** Usa `crate::config::postgresql_conmection::get_db().await?`. No crees pools adicionales por operación ni leas `DATABASE_URL` dentro de cada función del repository; la conexión global diferida ya está centralizada en configuración.

5. **Escribe SQL seguro y explícito.**
   - Usa `sqlx::query(...)` con placeholders de PostgreSQL (`$1`, `$2`, ...) y enlaza cada valor con `.bind(...)`.
   - Nunca concatenes datos externos dentro de SQL. Los identificadores de tabla o columna no se parametrizan; elige esos nombres en código y valida cualquier diseño dinámico.
   - Especifica las columnas necesarias en `SELECT` y `RETURNING` en vez de depender de `SELECT *`.
   - Para create/update, enlaza los campos en el mismo orden que los placeholders y usa `RETURNING` si la operación debe devolver el registro guardado.
   - Usa `fetch_one`, `fetch_optional`, `fetch_all` o `execute` según cardinalidad y resultado esperados.

6. **Mapea filas al dominio sin ocultar errores.** Sigue el patrón `fn row_to_<recurso>(row: PgRow) -> Result<Model, sqlx::Error>` con `Row::try_get`. Especifica tipos explícitos cuando el tipo no sea obvio, por ejemplo `NaiveDateTime`. Para varias filas, mapea y recolecta como `Result<Vec<_>, _>` para propagar errores de conversión. No uses `unwrap`, valores por defecto silenciosos ni ignores columnas que el modelo necesita.

7. **Maneja ausencia y filas afectadas correctamente.** Usa `fetch_optional` para `find_by_id` y para updates que pueden no encontrar el UUID. En `delete`, retorna `result.rows_affected() > 0`. No conviertas una fila inexistente en error de base de datos; permite que el service decida cómo representar el `None` o `false`.

8. **Actualiza el esquema documentado si cambia.** El proyecto no ejecuta migraciones automáticamente. Si una operación requiere una columna, índice, restricción o tabla nueva, modifica el SQL de `README.md` y describe cualquier paso requerido para bases existentes. Alinea tipos SQL con los tipos Rust del dominio y con las features SQLx habilitadas en `Cargo.toml`.

9. **Añade pruebas de persistencia apropiadas.** Revisa `tests/database.rs`, que está ignorada por requerir PostgreSQL y `DATABASE_URL`. Prueba las operaciones nuevas con una base de datos de prueba y datos aislados; limpia los datos creados cuando corresponda. No hagas que `cargo test` normal dependa de una base externa ni ejecutes pruebas concurrentes que compartan registros mutables sin aislamiento.

10. **Verifica los cambios.** Ejecuta los chequeos adecuados:

   ```bash
   cargo fmt --check
   cargo check
   cargo test
   ```

   Si hay PostgreSQL de prueba disponible y el cambio toca persistencia, ejecuta además la prueba ignorada o las pruebas de integración correspondientes. No informes que una prueba con base de datos pasó si no la ejecutaste.

## Reglas del proyecto

- Usa PostgreSQL y SQLx ya declarados en `Cargo.toml`; no introduzcas ORM o dependencias nuevas sin un requisito concreto.
- Mantén SQL parametrizado con `.bind(...)`; no registres credenciales ni datos sensibles.
- Retorna errores SQLx con `?` y deja el mapeo de errores de aplicación al service.
- Mantén sincronizados el repository, el modelo de dominio y el esquema SQL documentado.
- No uses migraciones implícitas ni afirmes que las tablas se crean automáticamente.
