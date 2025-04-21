use std::env;
#[derive()]
pub struct Cli{
  cmd_path:String,
  conn_path: String,
}
/// `Cli` 结构体提供了从命令行参数中获取命令路径和连接路径的方法。
///
/// # 方法
///
/// - `get_cmd_path(&self) -> &String`:
///   返回命令路径字符串的引用。
///
/// - `get_conn_path(&self) -> &String`:
///   返回连接路径字符串的引用。
///
/// - `cmd() -> Cli`:
///   通过读取命令行参数中的命令路径和连接路径来构造一个 `Cli` 实例。
///   如果没有提供参数，则为相应字段分配默认的错误消息。
impl Cli {
  pub fn get_cmd_path(&self) -> &String{
      let string_pattern = &self.cmd_path;
      string_pattern
  }
  pub fn get_conn_path(&self) -> &String{
      let string_path = &self.conn_path;
      string_path
  }

  pub fn cmd()-> Cli {
          let comm = env::args().nth(1);
          let _cmd_path = match comm {
            Some(s) => s,
            None => String::from("必须传入命令文件路径参数")
          };
          let conn_path = env::args().nth(2);
          let _conn_path = match conn_path {
            Some(s) => s,
            None => String::from("必须传入连接信息文件路径参数")
              
          };
          //println!("{:?},{:?}",_pattern,_path);
          Cli {cmd_path: _cmd_path,conn_path:_conn_path}
   } 
}