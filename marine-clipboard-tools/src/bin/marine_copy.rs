use libc::{STDIN_FILENO, STDOUT_FILENO};
use nix::{
    fcntl::OFlag,
    unistd::{close, dup2, fork, ForkResult},
};
use wayland_clipboard_listener::{WlClipboardCopyStream, WlClipboardListenerError};

fn main() -> Result<(), WlClipboardListenerError> {
    let args = std::env::args();
    if args.len() != 2 {
        println!("You need to pass a string to it");
        return Ok(());
    }
    let context: &str = &args.last().unwrap();
    let mut stream = WlClipboardCopyStream::init()?;

    if let Ok(ForkResult::Child) = unsafe { fork() } {
        if let Ok(dev_null) =
            nix::fcntl::open("/dev/null", OFlag::O_RDWR, nix::sys::stat::Mode::empty())
        {
            let _ = dup2(dev_null, STDIN_FILENO);
            let _ = dup2(dev_null, STDOUT_FILENO);
            let _ = close(dev_null);
            stream.copy_to_clipboard(context.as_bytes().to_vec(), false)?;
        }
    }

    Ok(())
}