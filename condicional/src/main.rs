fn main() {
    println!("Esta es tu última oportunidad. Después, ya no podrás echarte atrás. Si tomas la pastilla azul, fin de la historia. Despertarás en tu cama y creerás lo que quieras creerte. Si tomas la roja, te quedas en el País de las Maravillas y yo te enseñaré hasta dónde llega la madriguera de conejos. Recuerda lo único que te ofrezco es la verdad. Nada más.");
    println!("Qué pastilla tomarás? roja o azul?");

    let mut opcion: String = String::new();
    std::io::stdin().read_line(&mut opcion).unwrap();
    let pastilla: &str = opcion.trim();

    if pastilla == "roja" {
        println!("Muy bien, Neo. Sígueme....");
    } else if pastilla == "azul" {
        println!("Como prefieras. No nos veremos nunca mas....");
    } else {
        println!("Tal parece que no eres el Neo que pensábamos.");
    }
}
