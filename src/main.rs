use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fmt;
use std::thread;


#[allow(dead_code)]
enum HttpStatus {
	Ok,
	Forbidden,
	Error,
	RateLimit,
}


impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP/1.1 {}\r\n\r\n", match *self {
        	HttpStatus::Ok => "200 OK",
        	HttpStatus::Forbidden => "403 Forbidden",
        	HttpStatus::Error => "500 Internal Server Error",
        	HttpStatus::RateLimit => "429 Too Many Requests",
    	})
    }
}

fn handle_connection(mut stream: TcpStream) {
	        let mut s  = [0; 512];
        stream.read(&mut s).unwrap();
        let s = String::from_utf8_lossy(&s[..]);

        println!("{:?}", stream.peer_addr());

        let mut first = match s.lines().next() {
        	Some(x) => x.split_whitespace(),
        	None => return,
        };

    	let method = match first.next() {
    		Some(x) => x,
    		None => {
    			write!(&stream, "{}No method", HttpStatus::Error).unwrap();
    			stream.flush().unwrap();
    			return;
			}
    	};

        let path = match first.next() {
        	Some(x) => x,
        	None => {
        		write!(&stream, "{}No path requested", HttpStatus::Error).unwrap();
    			stream.flush().unwrap();
    			return;
        	}
        };

        if path.find("api").is_some() {
        	if let Some(endpoint) = path.split('/').last() {
        		println!("{} {}", method, endpoint);
        		write!(&mut stream, "{}", HttpStatus::Ok).unwrap();
        		write!(&stream, "endpoint: {}", endpoint).unwrap();

        		let mut parse = endpoint.split('?');
        		if let Some(function) = parse.next() {
        			if let Some(arguments) = parse.next() {
        				for args in arguments.split('&') {
	        				println!("{} {}", function, args);
	        			}
        			}

        		}
        	}
        } else {
        	println!("Not a valid API endpoint: {}", &path);
    	 	write!(&stream, "{}Not a valid API endpoint: {}", HttpStatus::Forbidden, &path).unwrap();
        }
	    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| handle_connection(stream.unwrap()));
    }
}
