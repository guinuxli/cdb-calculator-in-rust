use std::io;

fn cdb_calc(vi: f64, taxa_cdi: f64, t: f64, p_cdi: f64) -> f64{
    // vi = valor investido
    // taxa_cdi = taxa anual CDI
    // t = dias úteis do período
    // p_cdi = percentual do CDI oferecido
    vi * (1.0 + taxa_cdi * (p_cdi / 100.0)).powf(t / 252.0)   
}

fn main() -> io::Result<()> {
    println!("=== Calculadora CDB ===");

    // valor do dinheiro investido
    println!("Insira o valor a ser investido.");
    let mut vi = String::new();
    io::stdin().read_line(&mut vi)?;
    
    let vi: f64 = vi.trim().parse().expect("Valor inválido");
    
    println!("Valor de investimento inserido: {vi}");
    
    // valor do cdi
    println!("Insira o valor da taxa do cdi.");
    let mut taxa_cdi = String::new();
    io::stdin().read_line(&mut taxa_cdi)?;
    
    let taxa_cdi: f64 = taxa_cdi.trim().parse().expect("Valor inválido");
    
    println!("Taxa do CDI inserida: {taxa_cdi}");
        
    // por quantos dias/tempo a ser investido
    println!("Por quantos dias deseja investir? (valor numérico).");
    let mut t = String::new();
    io::stdin().read_line(&mut t)?;
    
    let t: f64 = t.trim().parse().expect("Valor inválido");
    
    println!("Dias: {t}");
    
    // taxa CDI oferecida na oferta. ex.: 102%
    println!("Insira o valor da oferta oferecida. ex.: 102%");
    let mut p_cdi = String::new();
    io::stdin().read_line(&mut p_cdi)?;
    
    let p_cdi: f64 = p_cdi.trim().parse().expect("Valor inválido");
    
    let resultado_final = cdb_calc(vi, taxa_cdi, t, p_cdi);

    let aliquota_ir: f64 = match t as u32 {
        0..=180   => 0.225,   // 22.5%
        181..=360 => 0.20,    // 20%
        361..=720 => 0.175,   // 17,5%
        _         => 0.15,    // 15% (acima de 720)
    };

    // lucro líquido
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
