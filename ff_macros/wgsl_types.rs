// macro_rules! string_to_field {
//     (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
//         pub struct $name {
//             $($fname : $ftype),*
//         }

//         impl WgslType for $name {
//               fn field_names() -> (&'static [&'static str],&'static [&'static
// str]) {                 static NAMES: &'static [&'static str] =
// &[$(stringify!($fname)),*];                 static TYPES: &'static [&'static
// str] = &[$(stringify!($ftype)),*];

//                 (NAMES, TYPES)
//             }
//             fn struct_name() -> &'static str {
//                 static NAME: &'static str = stringify!($name);

//                 NAME
//             }
//             fn new_wgsl_block(group: i32, binding: i32) -> String {
//                 let (field_names, field_types) = Self::field_names();
//                 let struct_name = Self::struct_name();
//                 // let name = format!("{:?}", T);
//                 let mut wgsl = String::new();
//                 wgsl.push_str(&format!("[[block]] \n struct {}
// \n",struct_name));

//                 for (name, typ) in field_names.iter().zip(field_types.iter())
// {                     let field_type = match *typ {
//                         "f32" => "float",
//                         "u32" => "u32",
//                         "i32" => "i32",
//                         "String" => "i32",
//                         _ => panic!("Unsupported type: {}", typ),
//                     };
//                     wgsl.push_str(&format!("    {} : {};\n", name,
// field_type));                 }
//                 wgsl.push_str(&format!(
//                     "[[group({}), binding({})]] \n var<uniform> {}: {}; \n",
//                     group,
//                     binding,
//                     struct_name.to_lowercase(),
//                     struct_name
//                 ));

//                 wgsl
//             }

//         }

//     }
// }

// pub trait WgslType {
//     fn field_names() -> (&'static [&'static str], &'static [&'static str]);
//     fn struct_name() -> &'static str;
//     fn new_wgsl_block(group: i32, binding: i32) -> String;
// }

// // pub fn new_wgsl_block<T: WgslType>(group: i32, binding: i32) -> String {
// //     let (field_names, field_types) = T::field_names();
// //     // let name = format!("{:?}", T);
// //     let mut wgsl = String::new();
// //     wgsl.push_str(&format!("[[block]] \n struct {} \n",
// T::struct_name()));

// //     for (name, typ) in field_names.iter().zip(field_types.iter()) {
// //         let field_type = match *typ {
// //             "f32" => "float",
// //             "u32" => "u32",
// //             "i32" => "i32",
// //             "String" => "i32",
// //             _ => panic!("Unsupported type: {}", typ),
// //         };
// //         wgsl.push_str(&format!("    {} : {};\n", name, field_type));
// //     }
// //     wgsl.push_str(&format!(
// //         "[[group({}), binding({})]] \n var<uniform> {}: {}; \n",
// //         group,
// //         binding,
// //         T::struct_name().to_lowercase(),
// //         T::struct_name()
// //     ));

// //     wgsl
// // }

// string_to_field! {
//  struct  MyStruct {
//     foo: i32,
//     bar: String
// }
// }
