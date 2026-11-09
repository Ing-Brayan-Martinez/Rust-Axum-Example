---
name: axum-implements-security
description: "Implementar o modificar seguridad y autenticación en esta API Rust/Axum: API_KEY, middleware, encabezado X-API-Key, protección de rutas, manejo de secretos y pruebas."
---

# Implementación de seguridad en Axum

Usa esta skill al cambiar autenticación, autorización, manejo de claves o protección de endpoints. El proyecto implementa una API key compartida con `API_KEY`, middleware Axum y comparación de tiempo constante; conserva este comportamiento salvo que el requerimiento pida explícitamente un modelo distinto.

## Procedimiento paso a paso

1. **Inspecciona la seguridad existente.** Revisa `src/config/api_key.rs`, `src/lib.rs`, `src/main.rs`, `tests/api_key.rs`, `tests/api_routes.rs`, `.gitignore`, `.env.example` y las instrucciones de configuración de `README.md`. Identifica rutas públicas y protegidas, cómo se carga la clave y todos los lugares donde el secreto podría exponerse.

2. **Define el límite de seguridad.** Determina qué rutas requieren autenticación, qué identidad o permiso se valida, cómo se transmite la credencial y qué respuestas reciben solicitudes ausentes o inválidas. Mantén `GET /` público y todas las rutas `/api/v1/*` protegidas por `X-API-Key`, a menos que el cambio autorice conscientemente una diferencia.

3. **Mantén la carga segura de configuración.**
   - Lee `API_KEY` después de cargar `.env`, mediante `load_api_key()` o una configuración centralizada equivalente.
   - Rechaza la variable ausente o vacía y propaga el error para impedir el arranque sin una clave válida.
   - No uses valores por defecto, claves embebidas en el código ni secretos versionados.
   - Si cambia la fuente del secreto, actualiza `.env.example`, README y los mecanismos de despliegue sin registrar su valor.

4. **Aplica middleware al grupo correcto de rutas.** Sigue la composición de `create_app` en `src/lib.rs`: combina los routers de API y aplica `require_api_key` a ese conjunto; registra la ruta pública `/` fuera del grupo protegido. Si agregas una ruta o router, verifica dónde se monta para evitar dejar endpoints sensibles sin middleware o proteger accidentalmente endpoints públicos.

5. **Valida credenciales sin filtrar información.**
   - Lee la credencial desde el encabezado `X-API-Key`; no la aceptes en URL, query string ni cuerpo JSON.
   - Rechaza encabezados ausentes, malformados o con clave incorrecta con una respuesta de no autorizado (`401`).
   - Conserva la comparación de tiempo constante de `subtle::ConstantTimeEq` en `api_key_matches`; no reemplaces la comparación por `==`, una comparación que salga al primer byte distinto o un hash improvisado.
   - No incluyas la clave esperada ni la recibida en el cuerpo de respuesta, logs, errores, trazas o mensajes de diagnóstico.

6. **No confundas autenticación y autorización.** La API key actual autentica el acceso a la API de forma compartida; no identifica usuarios individuales ni define roles. Si el requerimiento necesita permisos por usuario, scopes, expiración o rotación, diseña ese cambio explícitamente y actualiza contratos y pruebas en vez de asumir que una sola clave proporciona esas capacidades.

7. **Añade pruebas de seguridad.** Extiende `tests/api_key.rs` y/o `tests/api_routes.rs` usando `tower::ServiceExt::oneshot`. Comprueba como mínimo:
   - La clave configurada permite el acceso al endpoint protegido.
   - Clave ausente e incorrecta reciben `401`.
   - `GET /` sigue siendo público.
   - Cada nuevo router sensible queda protegido.
   - La carga de configuración rechaza clave ausente o vacía si se modifica su comportamiento; evita pruebas frágiles que dependan del entorno global compartido.

8. **Actualiza documentación y ejemplos.** Documenta el requisito de `API_KEY`, el encabezado `X-API-Key`, las rutas protegidas y las rutas públicas. Usa una clave aleatoria de ejemplo solo como instrucción de generación, nunca como credencial real. Indica el uso de HTTPS en despliegues remotos y no recomiendes enviar claves por URL.

9. **Verifica el cambio.** Ejecuta:

   ```bash
   cargo fmt --check
   cargo check
   cargo test
   ```

   Ejecuta pruebas adicionales pertinentes si cambió el middleware o el montaje de routers. No informes que una validación pasó si no la ejecutaste.

## Reglas de seguridad

- Nunca imprimas, registres ni incluyas secretos en commits, respuestas HTTP, capturas, documentación o resultados de pruebas.
- No elimines la comparación de tiempo constante ni relajes la validación obligatoria de `API_KEY` inadvertidamente.
- No devuelvas detalles internos al cliente; los fallos de autenticación deben responder de forma genérica.
- Mantén TLS/HTTPS en el borde para proteger credenciales en tránsito en entornos reales.
- Evita dependencias o esquemas nuevos de autenticación salvo que el requisito lo justifique.
