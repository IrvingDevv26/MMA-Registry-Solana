🥋 MMA Academy Ledger – Solana Smart Contract
📝 Resumen del Proyecto
MMA Academy Ledger es un programa descentralizado desarrollado con el Anchor Framework en la blockchain de Solana. Su función principal es permitir que entrenadores y dueños de gimnasios gestionen un registro profesional de sus atletas de forma inmutable.

A diferencia de un sistema tradicional, toda la información de los peleadores (récord, estilo y contacto) se almacena on-chain dentro de una cuenta segura y verificable.

🏗️ Arquitectura del Proyecto
Este repositorio está diseñado para trabajar en el entorno de Solana Playground y sigue la estructura estándar de Anchor:

src/lib.rs: Contiene el Smart Contract escrito en Rust con las reglas de negocio y seguridad.

client/client.ts: Script de TypeScript para interactuar con las funciones del contrato desde el navegador.

tests/: Carpeta para pruebas unitarias que validan el comportamiento del programa.

🧠 Funcionamiento de las PDAs
Cada usuario posee una Academia única vinculada a su llave pública mediante una cuenta PDA (Program Derived Address).

Semillas utilizadas:
seeds = ["dojo", owner_public_key]

Esto garantiza que:

Cada entrenador tiene un solo registro centralizado.

Nadie más puede modificar los datos de una academia que no le pertenece.

La cuenta se deriva de forma determinística sin necesidad de guardar direcciones adicionales.

📦 Estructuras de Datos
Academia
Es la cuenta principal que almacena el inventario de atletas.

owner: Llave pública del administrador.

nombre: Nombre oficial del gimnasio.

peleadores: Vector (Vec) que contiene hasta 10 perfiles detallados.

Peleador
Representa la ficha técnica de un atleta.

Datos Personales: Nombre, Apodo, Origen y Contacto.

Estadísticas: Victorias, Derrotas, Empates y KOs (almacenados como u16).

Estado: Valor booleano para indicar si el peleador está activo o retirado.

🚀 Instrucciones del Programa
El contrato expone las siguientes capacidades técnicas:

1️⃣ crear_academia
Inicializa el registro del dojo en la red. Establece al firmante como el único dueño con permisos de escritura.

2️⃣ agregar_peleador
Inserta un perfil completo en el roster. Permite definir el estilo de pelea (Striker, Grappler, etc.) y los medios de contacto desde el registro inicial.

3️⃣ alternar_estado
Cambia la disponibilidad del peleador. Si un atleta se lesiona o se retira, su estado cambia a inactivo sin borrar su historial de la cuenta.

4️⃣ eliminar_peleador
Remueve permanentemente a un peleador del vector, liberando espacio en la cuenta de la academia.

5️⃣ ver_peleadores
Función de auditoría que imprime el estado actual de la academia en los logs para verificar la integridad de la información.

🔐 Seguridad y Validaciones
Signer Validation: Todas las funciones de edición requieren que la wallet que firma sea el owner original de la academia.

Cálculo de Espacio: Se utiliza InitSpace para reservar exactamente la memoria necesaria, optimizando los costos de almacenamiento en Solana.

Manejo de Errores: Se incluyen errores personalizados como NoEresElOwner y PeleadorNoExiste para evitar transacciones inválidas.

🛠️ Cómo ejecutar en Solana Playground
Abrir SolPG: Entra a beta.solpg.io.

Importar: Copia el código de lib.rs en la carpeta src.

Compilar: Presiona Build.

Desplegar: Presiona Deploy en la red de Devnet.

Interacción: Usa la pestaña de instrucciones o el script en client.ts para realizar pruebas.
