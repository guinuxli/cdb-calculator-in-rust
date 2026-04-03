use std::io;

// ============================================================
// CALCULADORA DE CDB — Simulador de Renda Fixa Brasileira
// ============================================================
//
// CDB (Certificado de Depósito Bancário) é um título de renda
// fixa brasileiro emitido por bancos. Sua rentabilidade geralmente
// é atrelada ao CDI (Certificado de Depósito Interbancário), que
// acompanha de perto a taxa básica de juros (SELIC).
//
// COMO FUNCIONA:
//   - Você investe um valor inicial em um CDB.
//   - O banco oferece um percentual do CDI (ex: 102% do CDI).
//   - O dinheiro rende diariamente em dias úteis (252 por ano).
//   - No resgate, o Imposto de Renda (IR) é descontado sobre o lucro,
//     com alíquota dependendo do tempo da aplicação.
//
// ============================================================

/// Calcula o valor final bruto de um investimento em CDB.
///
/// # Parâmetros
/// - `vi`          : Valor investido (R$)
/// - `taxa_cdi`    : Taxa anual do CDI em decimal (ex: 0.1065 para 10,65%)
/// - `t`           : Número de dias úteis de aplicação
/// - `p_cdi`       : Percentual do CDI oferecido pelo banco (ex: 102.0 para 102%)
///
/// # Fórmula
/// Usa juros compostos ajustados para a convenção brasileira de dias úteis (252 dias/ano):
///
///   valor_final = principal * (1 + taxa_cdi * p_cdi/100) ^ (t (dias úteis) / 252)
///
/// Observação: Este é um modelo simplificado. Cálculos reais de CDB usam
/// capitalização diária com a taxa CDI de cada dia divulgada pela B3.

fn cdb_calc(vi: f64, taxa_cdi: f64, t: f64, p_cdi: f64) -> f64 {
    vi * (1.0 + taxa_cdi * (p_cdi / 100.0)).powf(t / 252.0)
}

// essa função server para o usuário ver
// a tabela de taxa do IR
// para saber o quanto vai descontar
// na quantidade de tempo escolhida
fn tabela_taxas() {
    println!("Tabela de Taxa do Imposto de Renda (IR).");
    println!("Até 180 dias úteis          -> 22,5%");
    println!("181 - 360 dias úteis        -> 20,0%");
    println!("361 - 720 dias úteis        -> 17,5%");
    println!("Mais de 720 dias úteis      -> 15,0%");
}

fn main() -> io::Result<()> {
    println!("=== Calculadora CDB ===");

    // chamando a tabela
    tabela_taxas();

    // input: valor do dinheiro investido
    println!("Insira o valor a ser investido. (BRL).");
    let mut vi = String::new();
    io::stdin().read_line(&mut vi)?;
    let vi: f64 = vi.trim().parse().expect("Valor inválido");
    println!(); // apenas para pular uma linha

    // input: valor da taxa selic do cdi
    // checar o valor oferecido pela B3.
    println!("Insira o valor da taxa do cdi. (ex.: 0.1451 para 14,51%)");
    let mut taxa_cdi = String::new();
    io::stdin().read_line(&mut taxa_cdi)?;
    let taxa_cdi: f64 = taxa_cdi.trim().parse().expect("Valor inválido");
    println!(); // apenas para pular uma linha

    // input: tempo investido. O dinheiro trabalha
    // em dias úteis, 252 dias por ano
    println!("Por quantos dias úteis você quer investir?");
    let mut t = String::new();
    io::stdin().read_line(&mut t)?;
    let t: f64 = t.trim().parse().expect("Valor inválido");
    println!(); // apenas para pular uma linha

    // taxa CDI oferecida na oferta do banco. ex.: 102%
    println!("Insira o valor da oferta oferecida. ex.: 102 para 102%");
    let mut p_cdi = String::new();
    io::stdin().read_line(&mut p_cdi)?;
    let p_cdi: f64 = p_cdi.trim().parse().expect("Valor inválido");
    println!(); // apenas para pular uma linha

    // Cálculo usando a função cdb_calc
    let resultado_final = cdb_calc(vi, taxa_cdi, t, p_cdi);

    // Determina o valor da taxa do IR seguindo
    // a tabela regresiva. Quanto mais dias úteis
    // passam, menor o valor da taxa
    let aliquota_ir: f64 = match t as u32 {
        0..=180 => 0.225,   // 22.5%
        181..=360 => 0.20,  // 20%
        361..=720 => 0.175, // 17,5%
        _ => 0.15,          // 15% (acima de 720)
    };

    // A taxa é aplicada apenas no valor de ganho
    // e não no valor investido
    let lucro = resultado_final - vi;
    let ir = lucro * aliquota_ir;
    let resultado_liquido = resultado_final - ir;
    let lucro_liquido = lucro - ir;

    println!("== RESULTADO ==");
    println!("Prazo:            {} dias", t);
    println!("Valor investido:  R$ {:.2}", vi);
    println!("Rendimento bruto: R$ {:.2}", resultado_final);
    println!("Lucro bruto:      R$ {:.2}", lucro);
    println!("Alíquota IR:      {:.1}%", aliquota_ir * 100.0);
    println!("IR descontado:    R$ {:.2}", ir);
    println!("Total líquido:    R$ {:.2}", resultado_liquido);
    println!("Lucro líquido:    R$ {:.2}", lucro_liquido);

    Ok(())
}
