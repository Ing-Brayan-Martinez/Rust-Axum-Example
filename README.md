# Rust-Axum-Example

[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
[![SQLx](https://img.shields.io/badge/SQLx-0.8-teal.svg)](https://github.com/launchbadge/sqlx)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-14%2B-blue.svg)](https://www.postgresql.org/)

API REST de ejemplo construida con **Rust**, **Axum** y **Tokio**. Implementa operaciones CRUD para tareas, productos y órdenes, con persistencia asíncrona en PostgreSQL a través de **SQLx** y una arquitectura en capas.

---

## 🚀 Características principales

- **API asíncrona:** Axum sobre Tokio.
- **Persistencia PostgreSQL:** consultas parametrizadas y pool de conexiones con SQLx.
- **Arquitectura en capas:** controladores HTTP, servicios, repositorios y modelos de dominio.
- **CRUD:** operaciones de creación, consulta, actualización y eliminación para tareas, productos y órdenes.
- **JSON:** los modelos de entrada y salida usan nombres de propiedades `camelCase`.

---

## 🛠️ Stack tecnológico

| Componente | Tecnología |
| :--- | :--- |
| Lenguaje | Rust (edición 2024) |
| Framework HTTP | Axum 0.8 |
| Runtime asíncrono | Tokio |
| Base de datos | PostgreSQL |
| Acceso a datos | SQLx 0.8 |
| Serialización | Serde |
| Valores decimales | `rust_decimal` |

---

## 📂 Arquitectura del proyecto

```text
Rust-Axum-Example/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    ├── main.rs                 # Inicialización del servidor y rutas
    ├── config/                 # Pool y configuración de PostgreSQL
    ├── controller/             # Endpoints y respuestas HTTP
    ├── domain/                 # Modelos y datos de entrada
    ├── repository/             # Consultas SQL
    └── service/                # Lógica de aplicación y manejo de resultados
```

---

## 📡 Endpoints

El puerto del servidor se configura con `PORT` (por defecto, `3000`). Las rutas CRUD están bajo `/api/v1`.

| Recurso | Método | Ruta | Descripción |
| :--- | :--- | :--- | :--- |
| Tareas | `GET` | `/api/v1/task` | Listar tareas |
| Tareas | `GET` | `/api/v1/task/{id}` | Obtener una tarea |
| Tareas | `POST` | `/api/v1/task` | Crear una tarea |
| Tareas | `PUT` | `/api/v1/task` | Actualizar una tarea |
| Tareas | `DELETE` | `/api/v1/task/{id}` | Eliminar una tarea |
| Productos | `GET` | `/api/v1/product` | Listar productos |
| Productos | `GET` | `/api/v1/product/{id}` | Obtener un producto |
| Productos | `POST` | `/api/v1/product` | Crear un producto |
| Productos | `PUT` | `/api/v1/product` | Actualizar un producto |
| Productos | `DELETE` | `/api/v1/product/{id}` | Eliminar un producto |
| Órdenes | `GET` | `/api/v1/order` | Listar órdenes |
| Órdenes | `GET` | `/api/v1/order/{id}` | Obtener una orden |
| Órdenes | `POST` | `/api/v1/order` | Crear una orden |
| Órdenes | `PUT` | `/api/v1/order/{id}` | Actualizar una orden |
| Órdenes | `DELETE` | `/api/v1/order/{id}` | Eliminar una orden |

Las búsquedas, actualizaciones o eliminaciones de registros inexistentes responden `404 Not Found`. Los errores de base de datos responden `500 Internal Server Error`.

### Ejemplos de cuerpos JSON

```json
// POST /api/v1/task
{
  "name": "Revisar el proyecto",
  "done": false
}
```

```json
// POST /api/v1/product
{
  "name": "Teclado",
  "price": 49.99,
  "stock": 12
}
```

```json
// POST /api/v1/order
{
  "name": "Compra de oficina",
  "amount": 149.97,
  "discount": 10.00
}
```

Para actualizar una tarea, envía `task_id`, `name` y `done` en el cuerpo de
`PUT /api/v1/task`. Para actualizar un producto, envía `productId`, `name`,
`price` y `stock` en el cuerpo de `PUT /api/v1/product`.

---

## ⚙️ Inicio rápido

### Requisitos

- Rust y Cargo (toolchain compatible con la edición 2024).
- Docker, o una instancia local de PostgreSQL.

### 1. Iniciar PostgreSQL con Docker

La aplicación lee `DATABASE_URL`, `PORT` y `API_KEY` del archivo `.env` en la raíz del proyecto. Copia `.env.example` a `.env` y ajusta la URL de PostgreSQL si es necesario:

```bash
cp .env.example .env
```

El ejemplo utiliza las credenciales de este contenedor local:

```bash
docker run --name rust-axum-postgres \
  -e POSTGRES_PASSWORD=tu_contraseña \
  -e POSTGRES_DB=tu_base_de_datos \
  -p 5432:5432 \
  -d postgres:14
```

Genera una clave aleatoria con `openssl rand -hex 32` y asígnala a `API_KEY` en `.env`:

```env
DATABASE_URL=postgres://usuario:contraseña@localhost/base_de_datos
PORT=3000
API_KEY=pega_aqui_la_clave_generada
```

La aplicación no inicia si `API_KEY` falta o está vacía. No subas `.env` al repositorio; está excluido por `.gitignore`. En otros entornos puedes definir estas variables directamente en el sistema.
Para probar desde otra terminal, exporta allí el mismo valor con `export API_KEY='tu_clave'`.

Todos los endpoints bajo `/api/v1/*` requieren la clave en el encabezado `X-API-Key`. El endpoint público `GET /` no requiere autenticación. No envíes la clave en la URL ni la registres en logs; usa HTTPS en entornos reales.

### 2. Crear las tablas

Conéctate a la base de datos y ejecuta el siguiente esquema. La aplicación no ejecuta migraciones automáticamente.

```sql
-- Para generar UUIDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
       
-- Tablas 
CREATE TABLE tasks (
    task_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE products (
    product_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    price NUMERIC NOT NULL,
    stock INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE orders (
    order_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    amount NUMERIC NOT NULL,
    discount NUMERIC NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

Las rutas de productos y tareas reciben UUID, al igual que las órdenes. Para bases
de datos existentes; el proyecto no aplica migraciones automáticamente.

### 3. Compilar, probar y ejecutar

```bash
cargo check
cargo test
cargo run
```

La API estará disponible en `http://localhost:<PORT>` (por defecto, `http://localhost:3000`). El endpoint `GET /` responde `Server is alive`.

Las pruebas de integración están en `tests/`. La prueba de conexión a PostgreSQL
está ignorada por defecto para no exigir una base de datos al ejecutar toda la
suite; para ejecutarla, configura `DATABASE_URL` y usa:

```bash
cargo test --test database -- --ignored
```

### Logs y diagnóstico

La aplicación usa `tracing` para registrar el inicio del servidor, errores de
servicio, duración de spans y solicitudes HTTP (método, ruta, estado y latencia).
El filtro predeterminado es `info`; puedes cambiarlo con `RUST_LOG`:

```bash
RUST_LOG=debug cargo run
```

Los valores de entrada no se registran. Las rutas se registran sin parámetros
de query para evitar exponer tokens u otros datos sensibles.

---

## 🧪 Prueba rápida

```bash
curl http://localhost:3000/api/v1/task \
  -H "X-API-Key: $API_KEY"
```

Crear una tarea:

```bash
curl -X POST http://localhost:3000/api/v1/task \
  -H 'Content-Type: application/json' \
  -H "X-API-Key: $API_KEY" \
  -d '{"name":"Primera tarea","done":false}'
```

---

## 📝 Notas

- La conexión PostgreSQL se inicializa de forma diferida al realizar la primera operación de repositorio.
- La conexión PostgreSQL se configura con `DATABASE_URL`, el puerto con `PORT` y la autenticación con `API_KEY`; pueden definirse en `.env`.
- Este proyecto no incluye actualmente Swagger/OpenAPI, Dockerfile de la aplicación ni configuración de Kubernetes.
