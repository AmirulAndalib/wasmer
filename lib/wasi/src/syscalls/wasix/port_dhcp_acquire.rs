use super::*;
use crate::syscalls::*;

/// ### `port_dhcp_acquire()`
/// Acquires a set of IP addresses using DHCP
pub fn port_dhcp_acquire(mut ctx: FunctionEnvMut<'_, WasiEnv>) -> Errno {
    debug!(
        "wasi[{}:{}]::port_dhcp_acquire",
        ctx.data().pid(),
        ctx.data().tid()
    );
    let env = ctx.data();
    let net = env.net();
    let tasks = env.tasks.clone();
    wasi_try!(__asyncify(&mut ctx, None, async move {
        net.dhcp_acquire().await.map_err(net_error_into_wasi_err)
    }));
    Errno::Success
}