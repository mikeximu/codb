
use codb::{Engine,DB};


fn main() -> Result<(), codb::DBError> {  // ✅ main 返回 Result，才能用 ?
    let db = Engine::new();

    db.put(b"a", b"1", None)?;
    let val = db.get(b"a", None)?;

    println!("value = {:?}", String::from_utf8_lossy(&val));  // ✅ 转成可打印字符串

    Ok(())
}