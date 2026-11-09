---
name: axum-implements-docker
description: "Dockerizar y ejecutar esta API Rust con Axum y PostgreSQL. Úsala para crear o modificar Dockerfile, .dockerignore, Docker Compose, imágenes, contenedores y configuración de despliegue local."
---

# Implementación de Docker

Usa esta skill para contenerizar el proyecto. La aplicación es una API Axum/Rust que escucha en `0.0.0.0:<PORT>`, requiere `API_KEY` al arrancar y accede a PostgreSQL de forma diferida con `DATABASE_URL`. Actualmente no hay Dockerfile ni Compose en el repositorio.

## Procedimiento paso a paso

1. **Inspecciona la configuración existente.** Revisa `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/config/`, `.env.example`, `.gitignore` y `README.md`. Confirma la versión de Rust, el puerto por defecto, variables requeridas y el esquema SQL vigente. No supongas que las migraciones se ejecutan automáticamente: este proyecto no las aplica.

2. **Elige los artefactos necesarios.** Para desarrollo o despliegue local con PostgreSQL, normalmente agrega un `Dockerfile`, `.dockerignore` y `compose.yaml`. Evita añadir Kubernetes, CI, scripts o servicios adicionales si no forman parte del requerimiento.

3. **Crea un Dockerfile multi-stage.**
   - Usa una imagen oficial de Rust como etapa de compilación y una imagen runtime compatible con el binario como etapa final.
   - Copia `Cargo.toml` y `Cargo.lock` para resolver y compilar dependencias de forma reproducible; luego copia el código fuente y compila en modo release.
   - Copia únicamente el ejecutable producido y las bibliotecas runtime necesarias a la imagen final; no incluyas el toolchain, cachés de Cargo ni código fuente innecesario.
   - Verifica el nombre real del binario derivado de Cargo; no lo infieras solo por el nombre de la librería. Asegúrate de que la imagen final tenga las bibliotecas del sistema requeridas por el binario elegido.
   - Ejecuta el proceso como usuario no root cuando sea posible. No incluyas claves, `.env` ni credenciales mediante `COPY`, `ARG` o valores embebidos.
   - Expón el puerto documentado como referencia; `EXPOSE` no publica por sí solo el puerto en el host.

4. **Crea `.dockerignore`.** Excluye al menos `target/`, `.git/`, `.idea/`, `.env`, artefactos locales y archivos temporales. No excluyas `Cargo.lock`, `Cargo.toml` ni los fuentes requeridos para compilar. Comprueba que `.env` no se envíe al contexto de build.

5. **Configura los servicios en Compose.**
   - Define un servicio para la API y otro para PostgreSQL, unidos por la red de Compose.
   - Publica el puerto de la API al host y configura `PORT` coherentemente con el puerto interno de escucha.
   - La URL de la API hacia la base de datos debe usar el nombre del servicio de PostgreSQL como host y el puerto interno `5432`; dentro del contenedor, `localhost` apunta al propio contenedor, no al servicio de base de datos.
   - Pasa `API_KEY` y credenciales mediante variables de entorno o un archivo local ignorado por Git; no las fijes en el Dockerfile ni en archivos versionados. Recuerda que el archivo `.env` usado por Compose para interpolación no se inyecta automáticamente al contenedor: declara explícitamente `environment` o `env_file`.
   - Agrega un volumen persistente para los datos de PostgreSQL y un healthcheck de base de datos si la configuración lo requiere. La mera condición `depends_on` no garantiza que PostgreSQL ya acepte conexiones.
   - Si se ejecuta la imagen oficial de PostgreSQL con inicialización de esquema, prepara un archivo SQL derivado del esquema documentado y móntalo en el directorio de inicialización de la imagen. Estos scripts solo se ejecutan al inicializar un directorio de datos vacío; no los presentes como migraciones continuas.

6. **Mantén fuera del repositorio los secretos locales.** Verifica que `.env` esté ignorado por Git. En ejemplos versionados, usa valores ficticios y documenta cómo generar una clave con `openssl rand -hex 32`. Para entornos compartidos o producción, usa el gestor de secretos del entorno y HTTPS; no uses claves de ejemplo.

7. **Construye y arranca.** Usa los comandos del proyecto desde la raíz:

   ```bash
   docker compose build
   docker compose up -d
   docker compose ps
   docker compose logs -f
   ```

   Si el código usa `.env` al arrancar, confirma que Compose suministra las variables requeridas al proceso del contenedor. Antes de probar CRUD, verifica que el esquema exista en la base de datos.

8. **Verifica la API y la persistencia.**
   - Prueba `GET /` en el puerto publicado; este endpoint es público.
   - Prueba una ruta `/api/v1/*` con `X-API-Key` configurada; sin la clave debe responder `401`.
   - Verifica que la API puede conectarse a PostgreSQL y que un CRUD funciona, sin exponer claves en comandos guardados, logs o salidas compartidas.
   - Comprueba la persistencia tras recrear el contenedor de PostgreSQL y revisa logs si la conexión falla.

9. **Actualiza documentación y valida.** Documenta en `README.md` la construcción, el arranque, las variables necesarias, el puerto, el esquema y los comandos para detener y limpiar los servicios. Advierte antes de cualquier comando que borre volúmenes o datos persistentes. Ejecuta `cargo fmt --check`, `cargo check` y `cargo test` para verificar el código; valida los artefactos Docker con `docker compose config` y una compilación de imagen si Docker está disponible.

## Consideraciones del proyecto

- La API usa `API_KEY` obligatoria, `PORT` (por defecto `3000`) y `DATABASE_URL`.
- Las rutas `/api/v1/*` están protegidas; `GET /` es público.
- La base de datos se inicializa de forma diferida al ejecutar operaciones de repositorio, así que el servidor puede iniciar antes de que PostgreSQL esté listo.
- El proyecto no contiene migraciones ni Dockerfile actualmente; no afirmes que el esquema se crea automáticamente salvo que el cambio lo implemente y documente.
- Nunca registres secretos, incluyas `.env` en la imagen o compartas credenciales reales.
