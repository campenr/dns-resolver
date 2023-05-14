// struct DNSHeader {
//     id: i32,
//     flags: i32,
//     num_questions: i32,
//     num_answers: i32,
//     num_authorities: i32,
//     num_additionals: i32,
// }
//
// impl DNSHeader {
//     pub fn new(id: i32, flags: i32) -> Self {
//         Self {
//             id,
//             flags,
//             num_questions: 0,
//             num_answers: 0,
//             num_authorities: 0,
//             num_additionals: 0,
//         }
//     }
// }
//
// struct DNSQuestion {
//     name: String,
//     type_: i32,
//     class_: i32,
// }

fn encode_dns_name(domain_name: &str) -> String {
    let mut encoded = String::new();
    let parts = domain_name.split('.');
    for s in parts {
        let s_len = format!("\\x{:02x}", s.len());
        encoded.push_str(&s_len);
        encoded.push_str(s);
    }
    encoded.push_str("\\x00");
    return encoded;
}

fn main() {

//     let header = new DNSHeader()

    let domain_name = "google.com";
    let encoded = encode_dns_name(domain_name);

    println!("{encoded}");
}
