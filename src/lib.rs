use anchor_lang::prelude::*;

// ID del Solana Program, este espacio se llena automáticamente al hacer el "build"
declare_id!("");

#[program] // Macro que convierte código de Rust a Solana. A partir de aquí empieza el código.
pub mod registro_mma {
    use super::*; // Importa todos los structs y enums definidos fuera del módulo

    //////////////////////////// Instrucción: Crear Academia /////////////////////////////////////
    /*
    Permite la creación de una PDA (Program Derived Address), un tipo especial de cuenta en Solana que permite prescindir 
    del uso de llaves privadas para la firma de transacciones. 

    Esta cuenta contendrá el objeto (struct) de tipo Academia donde podremos almacenar los Peleadores. 
    La creación de la PDA depende de 3 cosas:
        * Wallet address (owner)
        * Program ID 
        * string representativo, en este caso "academia"
    
    Parámetros de entrada:
        * nombre -> nombre de la academia -> tipo string
     */
    pub fn crear_academia(context: Context<NuevaAcademia>, nombre: String) -> Result<()> {
        // "Context" siempre suele ir como primer parámetro, ya que permite acceder al objeto o cuenta con el que queremos interactuar
        let owner_id = context.accounts.owner.key(); // Accedemos al wallet address del caller 
        msg!("Owner id: {}", owner_id); // Print de verificación

        let peleadores: Vec<Peleador> = Vec::new(); // Crea un vector vacío 

        // Creamos un Struct de tipo academia y lo guardamos directamente 
        context.accounts.academia.set_inner(Academia { 
            owner: owner_id,
            nombre,
            peleadores,
        });
        Ok(()) // Representa una transacción exitosa 
    }

    //////////////////////////// Instrucción: Agregar Peleador /////////////////////////////////////
    /*
    Agrega un peleador con todos los detalles de su perfil de combate al vector contenido en el struct Academia. 
    NuevoPeleador permite crear y modificar los valores relacionados a cualquier struct de tipo Peleador.

    Parámetros de entrada:
        * nombre, apodo, origen, estilo, contacto -> tipo string
        * victorias, derrotas, empates, kos -> tipo u16
     */ 
    pub fn agregar_peleador(
        context: Context<NuevoPeleador>, 
        nombre: String, 
        apodo: String,
        origen: String,
        estilo: String,
        contacto: String,
        victorias: u16,
        derrotas: u16,
        empates: u16,
        kos: u16
    ) -> Result<()> {
        require!( // Medida de seguridad para identificar que SOLO el owner de la academia sea el que hace cambios en ella
            context.accounts.academia.owner == context.accounts.owner.key(), // Condición: true -> continúa, false -> error
            Errores::NoEresElOwner 
        ); 

        let peleador = Peleador { // Creación del struct tipo Peleador
            nombre,
            apodo,
            origen,
            estilo,
            contacto,
            victorias,
            derrotas,
            empates,
            kos,
            activo: true, // El peleador se registra como activo por defecto
        };

        context.accounts.academia.peleadores.push(peleador); // Agrega el Peleador al vector

        Ok(()) 
    }

    //////////////////////////// Instrucción: Eliminar Peleador /////////////////////////////////////
    /*
    Elimina un peleador a partir de su nombre. Error si el peleador no existe. 

    Parámetros de entrada:
        * nombre -> Nombre del peleador -> string
     */
    pub fn eliminar_peleador(context: Context<NuevoPeleador>, nombre: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.academia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let peleadores = &mut context.accounts.academia.peleadores; // Referencia mutable al vector

        for i in 0..peleadores.len() { // Itera mediante el índice en busca del peleador a eliminar
            if peleadores[i].nombre == nombre { // Si lo encuentra, procede a borrarlo
                peleadores.remove(i);
                msg!("Peleador {} eliminado!", nombre); // Mensaje de borrado exitoso
                return Ok(()); 
            }
        }
        Err(Errores::PeleadorNoExiste.into()) // Transacción fallida, no encontró al peleador
    }

    //////////////////////////// Instrucción: Ver Peleadores /////////////////////////////////////
    /*
    Muestra en el log de la transacción el contenido completo del vector de peleadores.
     */
    pub fn ver_peleadores(context: Context<NuevoPeleador>) -> Result<()> {
        require!( // Medida de seguridad 
            context.accounts.academia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // :#? permite la visualización completa del vector en el log (requiere Debug)
        msg!("La lista de peleadores actualmente es: {:#?}", context.accounts.academia.peleadores); 
        Ok(()) 
    }

    //////////////////////////// Instrucción: Alternar Estado /////////////////////////////////////
    /* Cambia el estado del peleador de inactivo (false) a activo (true) o viceversa.

    Parámetros de entrada:
        * nombre -> Nombre del peleador -> string
     */
    pub fn alternar_estado(context: Context<NuevoPeleador>, nombre: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.academia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let peleadores = &mut context.accounts.academia.peleadores; 
        for i in 0..peleadores.len() { 
            let estado = peleadores[i].activo;  

            if peleadores[i].nombre == nombre { 
                let nuevo_estado = !estado;
                peleadores[i].activo = nuevo_estado;
                msg!("El peleador: {} ahora tiene un valor de actividad: {}", nombre, nuevo_estado); 
                return Ok(()); 
            }
        }

        Err(Errores::PeleadorNoExiste.into()) 
    }
}

/*
Códigos de error
Todos los códigos se almacenan en un enum con la siguiente estructura:
#[msg("MENSAJE DE ERROR")] 
NombreDelError,
*/
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la academia que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el peleador con el que deseas interactuar no existe")]
    PeleadorNoExiste,
}

#[account] // Especifica que el struct es una cuenta que se almacenará en la blockchain
#[derive(InitSpace)] // Genera la constante INIT_SPACE y determina el espacio de almacenamiento necesario 
pub struct Academia { // Define la Academia (cuenta principal)
    owner: Pubkey, // Pubkey es un formato de llave pública de 32 bytes 

    #[max_len(60)] // Cantidad máxima de caracteres
    nombre: String,

    #[max_len(10)] // Tamaño máximo del vector peleadores 
    peleadores: Vec<Peleador>,
}

/*
Struct secundario para definir el perfil de un peleador. 
Nota técnica: Se definen límites de caracteres (max_len) para cada String para calcular 
correctamente el espacio en la blockchain (InitSpace).
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Peleador {
    #[max_len(50)]
    nombre: String,

    #[max_len(30)] // Ejemplo: "El Matador"
    apodo: String,

    #[max_len(40)] // Ejemplo: "Alicante, España" o "Tijuana, MX"
    origen: String,

    #[max_len(30)] // Ejemplo: "Jiu-Jitsu / Boxeo"
    estilo: String,

    #[max_len(50)] // Ejemplo: "@fightermma" o "contacto@correo.com"
    contacto: String,

    // Datos numéricos (u16)
    victorias: u16, 
    derrotas: u16,
    empates: u16,
    kos: u16,

    // Estado en el roster
    activo: bool,
}

// Creación de los contextos para las instrucciones
#[derive(Accounts)] // Especifica las cuentas necesarias para la instrucción de creación
pub struct NuevaAcademia<'info> { 
    #[account(mut)] 
    pub owner: Signer<'info>, // Quien paga la transacción (mut para cambiar balance)

    #[account(
        init, // Indica que se creará una cuenta
        payer = owner, // Quien paga la renta de la cuenta 
        space = Academia::INIT_SPACE + 8, // Cálculo del espacio (8 bytes para el discriminador de Anchor)
        seeds = [b"dojo", owner.key().as_ref()], // PDA basada en un string y el id del owner
        bump // Metodo para derivar la PDA de forma segura
    )]
    pub academia: Account<'info, Academia>, // La cuenta creada almacenará el struct Academia

    pub system_program: Program<'info, System>, // Programa del sistema necesario para crear cuentas
}

// Contexto para la modificación de peleadores dentro de la academia existente
#[derive(Accounts)] 
pub struct NuevoPeleador<'info> {
    pub owner: Signer<'info>, // Quien firma la transacción

    #[account(mut)] 
    pub academia: Account<'info, Academia>, // Mutable porque modificaremos su vector interno
}
