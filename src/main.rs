#[macro_use]
extern crate wayland_client;

fn main() {
    use wayland_client::protocol::wl_output::WlOutput;
    use wayland_client::protocol::wl_seat::WlSeat;
    use wayland_client::Display;
    use wayland_client::GlobalManager;
    use wayland_client::Main;

    use std::sync::Arc;
    use std::sync::Mutex;

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

    use wayland_client::protocol::wl_shm::WlShm;
    use wayland_client::protocol::wl_shm::Format;
    use wayland_client::protocol::wl_shm;
    let shm : Main<WlShm> = globals.instantiate_exact(1).expect("There is no wl_shm?!");

    let mut formats = Arc::new(Mutex::new(Vec::new()));
    let formats2 = formats.clone();

    shm.quick_assign(move | shm, event, ddata|{
        match event {
            wl_shm::Event::Format{format} => {
                formats2.lock().unwrap().push(format);
            },
            _ => ()
        }
    });

    event_queue
        .dispatch(&mut (), |_, _, _| {
            /* This closure will be called for every event received by an object not
            assigned to any Filter. If you plan to assign all your objects to Filter,
            the simplest thing to do is to assert this is never called. */
            //unreachable!();
            println!("AAh event");
        })
        .expect("An error occurred during event dispatching!");

    for f in formats.lock().unwrap().iter() {
        println!("Format: {:?}", f);
    }

    let mem_file = memfd::MemfdOptions::new().close_on_exec(true).create("Wayland Client file desc").expect("failed to create fd").into_file();

    let pool_size : i32 = 64 * 64 * 4;

    mem_file.set_len(pool_size as u64);
    use std::os::unix::io::IntoRawFd;
    let memfd = mem_file.into_raw_fd();

    let mapping = mmap::MemoryMap::new(pool_size as usize, &[mmap::MapOption::MapFd(memfd)]).expect("Aah failed to map fd");
    let map_ptr = mapping.data();
    let map_len = mapping.len();

    let shm_pool = shm.create_pool(memfd, pool_size);

    let buffer = shm_pool.create_buffer(0, 64, 64, 64 * 4, Format::Argb8888);

    let surface = compositor.create_surface();

    let xdg_surface = xdg_shell.get_xdg_surface(&surface);

    use wayland_protocols::xdg_shell::client::xdg_toplevel::XdgToplevel;
    let toplevel = xdg_surface.get_toplevel();

    xdg_surface.set_window_geometry(0,0, 256, 256);

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
