use std::io::{ Read, };
use std::net::{ TcpListener, TcpStream };
use std::str::SplitWhitespace;

mod logger;

mod method;
use method::*;

mod router;
use router::RouteStruct;

mod util;

fn main() {
    const PORT: i32 = 3000;
    let mut route: RouteStruct = router::main();
    let (mut logger, _) = logger::new();

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    // let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000);

    // match mysql.stdin.unwrap().write("select * from user_info".as_bytes()) {
    //     Ok(_) => {
    //         println!("asdf");
    //     },
    //     Err(error) => {
    //         println!("{}", error);
    //     }
    // };

    logger.yellow().log(&format!("Server is Established at port: {}", PORT));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 5120];

        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap();
        let mut request_type = "Request";

        for header in request.split("\n") {
            let mut header = header.split(": ");

            if header.nth(0).unwrap() == "Content-Type" {
                request_type = "Json Request";
            }
        }

        let request_type = logger.bright_green().get_color_text(request_type);
        let color_text = logger.yellow().get_color_text("[");

        logger.log(&format!("{} {}", request_type, color_text));
        
        let parts = &mut request_line.split_whitespace();
        
        handle_connection(stream, &mut route, parts);
    }
}

fn handle_connection(stream: TcpStream, route: &mut RouteStruct, parts: &mut SplitWhitespace) {
    let (mut logger, mut error_logger) = logger::new();

    match parts.next() {
        Some(method) => {
            logger.log(&format!("   Method: {}", method));

            let method: Method = get_method(method);

            match parts.next() {
                Some(url) => {
                    logger.log(&format!("   Url: {}", url));

                    if route.is_ignore_url(&url) {
                        logger.log("   \x1b[33mError:\x1b[0m \x1b[31mNot Allowed Request Url\x1b[0m");
                        logger.log("]");

                        return;
                    };

                    route.call_router(stream, method, url);
                },
                None => {
                    logger.log("]");
                    error_logger.log("Request url is not allowed");
                }
            }
        },
        None => {
            logger.log("]");
            error_logger.log("Http Method is not support");
        }
    }
}