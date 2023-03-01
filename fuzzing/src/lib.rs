use num_bigint::BigInt;
use num_traits::Num;

pub mod math;

#[macro_export]
macro_rules! test_template_file {
    ($template_file:literal, $ctx:ident) => {{
        let template =
            std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../core/tests/templates/", $template_file));
        let mut tt = tinytemplate::TinyTemplate::new();
        let template = std::fs::read_to_string(template).unwrap();
        tt.add_template("test", &template).unwrap();
        tt.render("test", &$ctx).unwrap().clone()
    }};
}

pub fn get_prime() -> BigInt {
    BigInt::from_str_radix("3618502788666131213697322783095070105623107215331596699973092056135872020481", 10).unwrap()
}
