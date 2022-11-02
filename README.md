# Kinematics Calculator (Calculadora de Cinemática)

## Code
```rust
use std::io;

fn show_options() {
    println!("=== CALCULADORA DE CINEMÁTICA ===
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
        3 => uniform_motion(),
        4 => uniformly_varied_motion(),
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
    println!("\nA = Δv / Δt\n");
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
    println!("\nV = Δs / Δt\n");
    println!("Qual a distância percorrida em metros?");
    let delta_s: f64 = read_number();
    println!("Qual o tempo percorrido em segundos?");
    let delta_t: f64 = read_number();
    let velocity: f64 = delta_s / delta_t;
    println!("V = Δs / Δt
V = {} m / {} s
V = {} m/s", delta_s, delta_t, velocity);
}

fn uniform_motion() {
    println!("\nS = S_0 + v.t\n");
    print!("Qual a posição inicial em metros?");
    let s_0: f64 = read_number();
    println!("Qual a velocidade em m/s?");
    let v: f64 = read_number();
    println!("Qual foi o tempo percorrido em segundos?");
    let t: f64 = read_number();
    let s: f64 = s_0 + v * t;
    println!("S = S_0 + v.t
S = {} m + {} m/s * {} s
S = {} m + {} m
S = {} m", s_0, v, t, s_0, (v * t), s);
}

fn uniformly_varied_motion() {
    println!("\nS = S_0 + vt + at²\n");
    println!("Qual a posição inicial em metros?");
    let s_0: f64 = read_number();
    println!("Qual a velocidade inicial em m/s?");
    let v_0: f64 = read_number();
    println!("Qual foi o tempo percorrido em segundos?");
    let t: f64 = read_number();
    println!("Qual foi a aceleração em m/s²?");
    let a: f64 = read_number();
    let s: f64 = s_0 + v_0 * t + a * (t * t);
    println!("S = S_0 + vt + at²");
    println!("S = {} m + {} m/s * {} s + {} m/s² * {} s²
S = {} m + {} m + {} m
S = {} m", s_0, v_0, t, a, (t * t), s_0, (v_0 * t), (a * t * t), s);
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
```
