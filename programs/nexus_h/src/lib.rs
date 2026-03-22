use anchor_lang::prelude::*;

declare_id!("CMtFocfm6X68u1sxxe9vo78YeuuKZ7q2EkpRNvBH29vt");

#[program]
pub mod nexus_h {
    use super::*;

    // Función para registrar a un paciente en la blockchain
    pub fn registrar_paciente(
        ctx: Context<RegistrarPaciente>, 
        biometria_hash: String, 
        sangre: String, 
        alergias: String,
        idioma: String
    ) -> Result<()> {
        let cuenta_paciente = &mut ctx.accounts.paciente;
        cuenta_paciente.biometria_hash = biometria_hash;
        cuenta_paciente.tipo_sangre = sangre;
        cuenta_paciente.alergias = alergias;
        cuenta_paciente.idioma = idioma;
        cuenta_paciente.autorizado = ctx.accounts.autor.key();
        
        msg!("Paciente registrado con éxito para la Hora Dorada.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegistrarPaciente<'info> {
    #[account(init, payer = autor, space = 8 + 64 + 8 + 200 + 50 + 32)]
    pub paciente: Account<'info, Paciente>,
    #[account(mut)]
    pub autor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Paciente {
    pub biometria_hash: String, // Simboliza el rostro/iris
    pub tipo_sangre: String,    // Dato crítico médico
    pub alergias: String,      // Evita errores fatales
    pub idioma: String,        // Inclusión para lenguas indígenas
    pub autorizado: Pubkey,     // Llave del dueño
} 