// Usage:
// 
// #[derive(APIErrors)]
// pub enum {System}APIErrors {
//     #[meta(system="", code="1", message="Successful.", status_code=200, pvlost=0)]
//     Successful,
//     #[meta(system="", code="2", message="Unknown.", status_code=500, pvlost=1)]
//     Unknown,
// }
//