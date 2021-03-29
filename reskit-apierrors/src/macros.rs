// Usage:
// 
// #[derive(APIErrorMeta)]
// pub enum {System}APIErrorMeta {
//     #[meta(system="", code="1", message="Successful.", status_code=200, pvlost=0)]
//     Successful,
//     #[meta(system="", code="2", message="Unknown.", status_code=500, pvlost=1)]
//     Unknown,
// }
//

// macro_rules! adapt_errorspace_with_skip {
//     () => {
//      
//     };
// }

// // equals to: adapt_errorspace(space, ...) = adapt_errorspace_with_skip!(space, 3, ...)
// macro_rules! adapt_errorspace {
//     () => {
//
//     };
// }

// // equals to: adapt_skip(skip, ...) = adapt_errorspace_with_skip("", skip, ...)
// macro_rules! adapt_skip {
//     () => {
//        
//     };
// }

// // equals to: adapt(...) = adapt_errorspace!("", ...)
// macro_rules! adapt {
//     () => {
//        
//     };
// }
