#[no_mangle]
extern "C" fn swi_handler(swi_code: u32, args: [u32]){
    match swi_code{
        0 => /** Fork */,
        1 => /** Exit */,
        2 => /** Read */,
        3 => /** Write */,
        4 => /** Open */,
        5 => /** Close */,
        6 => /** Wait */,
        _ => ,
    }
}