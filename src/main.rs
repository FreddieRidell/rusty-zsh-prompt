fn get_host_name(): str {
 "%{%F{red}%}%m";
}

fn main() {
    println!("{}", get_host_name());
}
