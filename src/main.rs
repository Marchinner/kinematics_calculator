use std::io;

fn show_options() {
    println!("=== CALCULADORA DE FÍSICA ===
(1) - Velocidade Escalar Média
(2) - Aceleração Escalar Média
(3) - Movimento Uniforme
(4) - Movimento Uniformente Variável
(5) - Converter km/h -> m/s
(6) - Converter m/s -> km/h
");
    let option: u8 = read_option();
    match option {
        1 => velocity(),
        2 => acceleration(),
        3 => println!("MU"),
        4 => println!("MUV"),
        5 => kmh_to_ms(),
        6 => ms_to_kmh(),
        _ => println!("Falha na leitura!"),
    };
}

fn read_option() -> u8 {
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .unwrap();
    let option: u8 = option.trim().parse().unwrap();
    option
}

fn read_number() -> f64 {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .unwrap();
    let number: f64 = number.trim().parse().unwrap();
    number
}

fn acceleration() {
    println!("Qual a velocidade INICIAL em m/s?");
    let v_0: f64 = read_number();
    println!("Qual a velocidade FINAL em m/s?");
    let v: f64 = read_number();
    println!("Qual o tempo INICIAL do deslocamento em segundos?");
    let t_0: f64 = read_number();
    println!("Qual o tempo FINAL do deslocamento em segundos?");
    let t: f64 = read_number();
    let delta_v: f64 = v - v_0;
    let delta_t: f64 = t - t_0;
    let a: f64 = (delta_v) / (delta_t);
    println!("A = Δv / Δt
A = {} m/s / {} s
A = {} m/s²", delta_v, delta_t, a);
}

fn velocity() {
    println!("Qual a distância percorrida em metros?");
    let delta_s: f64 = read_number();
    println!("Qual o tempo percorrido em segundos?");
    let delta_t: f64 = read_number();
    let velocity: f64 = delta_s / delta_t;
    println!("V = Δs / Δt
V = {} m / {} s
V = {} m/s", delta_s, delta_t, velocity);
}

fn kmh_to_ms() {
    println!("Velocidade em km/h:");
    let kmh: f64 = read_number();
    let ms: f64 = kmh / 3.6;
    println!("m/s = km/h / 3.6
m/s = {} / 3.6
m/s = {:.2}", kmh, ms);
}

fn ms_to_kmh() {
    println!("Velocidade em m/s:");
    let ms: f64 = read_number();
    let kmh: f64 = ms * 3.6;
    println!("km/h = m/s * 3.6
km/h = {} m/s * 3.6
km/h = {:.02}", ms, kmh);
}

fn main() {
    show_options();
}
