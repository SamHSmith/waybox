#[macro_use]
extern crate wayland_client;

fn main() {
    use wayland_client::protocol::wl_output::WlOutput;
    use wayland_client::protocol::wl_seat::WlSeat;
    use wayland_client::Display;
    use wayland_client::GlobalManager;
    use wayland_client::Main;

    let _display = Display::connect_to_env().expect("Failed to connect to wayland server.");

    let mut event_queue = _display.create_event_queue();

    let display_attach = _display.attach(event_queue.token());

    let globals = GlobalManager::new_with_cb(
        &_display.attach(event_queue.token()),
        global_filter!(
            // Bind all wl_seat with version 4
            [
                WlSeat,
                4,
                |main: Main<WlSeat>, ddata: wayland_client::DispatchData| {
                    println!("seat");
                }
            ],
            // Bind all wl_output with version 1
            [
                WlOutput,
                1,
                |main: Main<WlOutput>, ddata: wayland_client::DispatchData| {
                    println!("output");
                }
            ]
        ),
    );

    event_queue
        .dispatch(&mut (), |_, _, _| {
            /* This closure will be called for every event received by an object not
            assigned to any Filter. If you plan to assign all your objects to Filter,
            the simplest thing to do is to assert this is never called. */
            //unreachable!();
            println!("AAh event");
        })
        .expect("An error occurred during event dispatching!");

    let globs = globals.list();
    for x in globs {
        println!("{} {} {}", x.0, x.1, x.2);
    }

    use wayland_client::protocol::wl_compositor::WlCompositor;
    let compositor: Main<WlCompositor> = globals.instantiate_exact(4).expect("Ehm, no compositor?");

    use wayland_protocols::xdg_shell::client::xdg_wm_base::XdgWmBase;
    let xdg_shell : Main<XdgWmBase> = globals.instantiate_exact(2).expect("There is no xdg_wm_base.");

    let surface = compositor.create_surface();

    let xdg_surface = xdg_shell.get_xdg_surface(&surface);

    use wayland_protocols::xdg_shell::client::xdg_toplevel::XdgToplevel;
    let toplevel = xdg_surface.get_toplevel();

    println!("Hello, world!");

    loop {
        // The dispatch() method returns once it has received some events to dispatch
        // and have emptied the wayland socket from its pending messages, so it needs
        // to be called in a loop. If this method returns an error, your connection to
        // the wayland server is very likely dead. See its documentation for more details.
        event_queue
            .dispatch(&mut (), |_, _, _| {
                /* This closure will be called for every event received by an object not
                assigned to any Filter. If you plan to assign all your objects to Filter,
                the simplest thing to do is to assert this is never called. */
                //unreachable!();
                println!("AAh event");
            })
            .expect("An error occurred during event dispatching!");
    }

    println!("Goodbye, world.");
}
