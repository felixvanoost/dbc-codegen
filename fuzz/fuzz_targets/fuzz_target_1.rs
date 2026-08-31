#![no_main]
use libfuzzer_sys::arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use cantools_messages::{
    example_bar_four_encode, example_bar_one_encode, example_bar_pack, example_bar_t,
    example_bar_three_encode, example_bar_two_encode, example_bar_type_encode,
};

fuzz_target!(|dbc_codegen_bar: can_messages::Bar| {
    let dbc_codegen_bar = can_messages::Bar::new(
        3,
        2.0,
        can_messages::BarThree::_Other(4),
        can_messages::BarFour::_Other(5),
        can_messages::BarType::X0off,
    )
    .unwrap();

    println!(
        "{} {} {} {} {}",
        dbc_codegen_bar.one(),
        dbc_codegen_bar.two(),
        u8::from(dbc_codegen_bar.three()),
        u8::from(dbc_codegen_bar.four()),
        bool::from(dbc_codegen_bar.xtype())
    );

    let one = unsafe { example_bar_one_encode(dbc_codegen_bar.one() as f64) };
    let two = unsafe { example_bar_two_encode(dbc_codegen_bar.two() as f64) };
    let three = unsafe { example_bar_three_encode(u8::from(dbc_codegen_bar.three()) as f64) };
    let four = unsafe { example_bar_four_encode(u8::from(dbc_codegen_bar.four()) as f64) };
    let type_ =
        unsafe { example_bar_type_encode(bool::from(dbc_codegen_bar.xtype()) as u8 as f64) };

    let bar = example_bar_t {
        one,
        two,
        three,
        four,
        type_,
    };
    let mut buffer: [u8; 8] = [0; 8];
    unsafe { example_bar_pack(buffer.as_mut_ptr(), &bar, buffer.len() as u64) };

    assert_eq!(dbc_codegen_bar.raw(), buffer);
});
