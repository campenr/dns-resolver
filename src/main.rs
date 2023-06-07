use rand::Rng;

struct DNSHeader {
    id: u32,
    flags: u32,
    num_questions: u32,
    num_answers: u32,
    num_authorities: u32,
    num_additionals: u32,
}

impl DNSHeader {
    pub fn new(id: u32, flags: u32) -> Self {
        Self {
            id,
            flags,
            num_questions: 0,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0,
        }
    }
    pub fn to_bytes(self) -> String {
        let mut encoded = String::new();
//         let id_bytes = u8::try_from(s_len).unwrap();
        let id_bytes = hex::encode(self.id);
//         let id_bytes = format!("\\x{}", &id_bytes);
        encoded.push_str(&format!("\\x{:02x}", self.id));
        print_type_of(&id_bytes);
        println!("{id_bytes}");
        return encoded;
    }
}

// struct DNSQuestion {
//     name: String,
//     type_: i32,
//     class_: i32,
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn encode_dns_name(domain_name: &str) -> String {
    let mut encoded = String::new();
    let parts = domain_name.split('.');
    for s in parts {
//         let s_len = format!("\\x{:02x}", hex::encode([s.len()]));
        let s_len = s.len();
        let s_len = u8::try_from(s_len).unwrap();
        let s_len = hex::encode(vec![s_len]);
        let s_len = format!("\\x{}", &s_len);
//         print_type_of(&s_len);
//         println!("{s_len}");
        encoded.push_str(&s_len);
        encoded.push_str(s);
    }
    encoded.push_str("\\x00");
    return encoded;
}

fn build_query() {
    let mut rng = rand::thread_rng();
    let id = rng.gen_range(0..65535);
    let header = DNSHeader::new(id, 1).to_bytes();

    println!("{header}");

    let domain_name = "google.com";
    let encoded = encode_dns_name(domain_name);


    println!("{encoded}");
}

fn main() {
    build_query();
}
