// 83. How do you implement a toy actor model with a single-threaded executor and a mailbox per actor?
// Implement ping-pong messages between actors. What are the benefits of the actor model?

// building a tiny actor system: each actor has its own mailbox (queue), processes messages one at a time, and interacts by sending messages.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Each actor has a mailbox of messages
type Message = Box<dyn FnOnce(&mut Context)>;

struct Mailbox {
    queue: VecDeque<Message>,
}

impl Mailbox {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    fn send(&mut self, msg: Message) {
        self.queue.push_back(msg);
    }
    fn receive(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }
}

// Context: handle passed into an actor so it can send to others
#[derive(Clone)]
struct Context {
    id: usize,
    system: ActorSystemHandle,
}

impl Context {
    fn send(&self, to: usize, msg: Message) {
        self.system.send(to, msg);
    }
}

// The actor system holds all mailboxes
struct ActorSystem {
    mailboxes: Vec<Mailbox>,
}

// Handle is a reference-counted pointer to the system
#[derive(Clone)]
struct ActorSystemHandle {
    inner: Rc<RefCell<ActorSystem>>,
}

impl ActorSystemHandle {
    fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ActorSystem { mailboxes: vec![] })),
        }
    }

    fn spawn(&self) -> Context {
        let mut sys = self.inner.borrow_mut();
        let id = sys.mailboxes.len();
        sys.mailboxes.push(Mailbox::new());
        Context {
            id,
            system: self.clone(),
        }
    }

    fn send(&self, to: usize, msg: Message) {
        let mut sys = self.inner.borrow_mut();
        sys.mailboxes[to].send(msg);
    }

    fn run(&self) {
        loop {
            let mut idle = true;
            let mut sys = self.inner.borrow_mut();

            for id in 0..sys.mailboxes.len() {
                if let Some(msg) = sys.mailboxes[id].receive() {
                    drop(sys); // release borrow before running msg
                    let mut ctx = Context {
                        id,
                        system: self.clone(),
                    };
                    msg(&mut ctx);
                    idle = false;
                    sys = self.inner.borrow_mut(); // reborrow after message
                }
            }

            if idle {
                break; // stop when all queues are empty
            }
        }
    }
}

// ---- Example: Ping-Pong ----
fn main() {
    let system = ActorSystemHandle::new();

    let ping = system.spawn();
    let pong = system.spawn();

    // Kick off ping
    ping.send(
        ping.id,
        Box::new({
            let pong_ctx = pong.clone();
            move |ctx| {
                println!("Ping: sending Ping -> Pong");
                ctx.send(
                    pong_ctx.id,
                    Box::new({
                        let ping_ctx = ctx.clone();
                        move |_pong_ctx| {
                            println!("Pong: received Ping, sending Pong -> Ping");
                            ping_ctx.send(
                                ping_ctx.id,
                                Box::new(|_ping_ctx| {
                                    println!("Ping: received Pong, done!");
                                }),
                            );
                        }
                    }),
                );
            }
        }),
    );

    system.run();
}
