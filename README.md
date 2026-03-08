# 🥋 MMA Registry – Solana

## 📝 Resumen del proyecto
**MMA Registry** es un programa descentralizado desarrollado con el **Anchor Framework** en la blockchain de **Solana**. Su función principal es permitir que entrenadores y dueños de gimnasios gestionen un registro profesional de sus atletas de forma inmutable.

A diferencia de un sistema tradicional, toda la información de los peleadores (récord, estilo y contacto) se almacena **on-chain** dentro de una cuenta segura y verificable.

---

## 🏗️ Arquitectura del proyecto
Este repositorio está diseñado para trabajar en el entorno de **Solana Playground** y sigue la estructura estándar de Anchor:

* **`src/lib.rs`**: Contiene el Smart Contract escrito en Rust con las reglas de negocio y seguridad.
* **`client/client.ts`**: Script de TypeScript para interactuar con las funciones del contrato desde el navegador.
* **`tests/`**: Carpeta para pruebas unitarias que validan el comportamiento del programa.

---

## 🧠 Funcionamiento de las PDAs
Cada usuario posee una **Academia** única vinculada a su llave pública mediante una cuenta PDA (Program Derived Address).

**Semillas utilizadas:**
`seeds = ["dojo", owner_public_key]`

Esto garantiza que:
1. Cada entrenador tiene **un solo registro centralizado**.
2. Nadie más puede modificar los datos de una academia que no le pertenece.
3. La cuenta se deriva de forma determinística sin necesidad de guardar direcciones adicionales.

---

## 📦 Estructuras de datos

### Academia
Es la cuenta principal que actúa como base de datos del gimnasio.
* **owner**: Llave pública del administrador.
* **nombre**: Nombre oficial de la institución.
* **peleadores**: Vector (`Vec`) que almacena la lista de atletas registrados.

### Peleador
Representa la ficha técnica y deportiva de un atleta.
* **Datos Personales**: Nombre, Apodo, Origen y Contacto.
* **Estadísticas**: Victorias, Derrotas, Empates y KOs almacenados como `u16`.
* **Estado**: Valor booleano para indicar si el peleador está activo o retirado.

---

## 🚀 Instrucciones del programa
El contrato expone las siguientes capacidades técnicas:

### 1️⃣ crear_academia
Inicializa el registro del dojo en la red y establece al firmante como el único dueño con permisos de escritura.

### 2️⃣ agregar_peleador
Inserta un perfil completo en el roster, permitiendo definir el estilo de pelea y medios de contacto desde el inicio.

### 3️⃣ alternar_estado
Permite modificar la disponibilidad del peleador (activo/inactivo) sin borrar su historial de la cuenta.

### 4️⃣ eliminar_peleador
Remueve permanentemente a un peleador del vector para mantener la base de datos eficiente.

---

## 🔐 Seguridad y validaciones
* **Signer Validation**: Todas las funciones de edición requieren que la wallet que firma sea el `owner` original.
* **Cálculo de Espacio**: Se utiliza `InitSpace` para reservar exactamente la memoria necesaria, optimizando costos en Solana.
* **Manejo de Errores**: Incluye errores personalizados como `NoEresElOwner` y `PeleadorNoExiste` para evitar operaciones inválidas.

---

## 🛠️ Cómo ejecutar en solana playground
1. **Abrir SolPG**: Entra a [beta.solpg.io](https://beta.solpg.io).
2. **Importar**: Copia el código de `lib.rs` en la carpeta `src`.
3. **Compilar**: Presiona el botón **Build**.
4. **Desplegar**: Presiona **Deploy** en la red de Devnet.
5. **Interacción**: Usa la pestaña de tests para realizar pruebas.

---

**Proyecto desarrollado por Irving de Jesus Davila Torres.**
