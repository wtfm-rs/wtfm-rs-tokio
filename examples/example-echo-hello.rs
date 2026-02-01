use tokio::process::Command;

async fn hello_world() -> String {
    let output = Command::new("echo").arg("Hello,").arg("world!").output();
    let output = output.await.expect("No such file or directory");
    String::from_utf8(output.stdout).expect("Format error")
}

#[tokio::main]
async fn main() {
    println!("{}", hello_world().await);
}
