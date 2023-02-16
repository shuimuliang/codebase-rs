// #[proc_macro_derive(NameFn)]
// pub fn derive_name_fn(items: TokenStream) -> TokenStream {
//
//     fn ident_name(item: TokenTree) -> String {
//         match item {
//             TokenTree::Ident(i) => i.to_string(),
//             _ => panic!("Not an ident")
//         }
//     }
//
//     let item_name = ident_name(items.into_iter().nth(1).unwrap());
//
//     format!("impl Name for {} {{
//    fn name() -> String {{
//        \"{}\".to_string()
//    }}
// }}", item_name, item_name).parse().unwrap()
// }
//
// #[derive(NameFn)]
// struct Info;

use bitflags::bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

fn main() {
    // println!("Hello, world!");
    // println!(Info::name());
    let _e1 = Flags::A | Flags::C;
}
