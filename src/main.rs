use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use webster::ThreadPool;

fn main() 
    {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let _pool = ThreadPool::new(4);

    /* .take(x)
    Will stop the for-loop after two requests.
     */
    for stream in listener.incoming().take(1)
        {
        /* unwrap
        For now, our handling of the stream consists of calling unwrap to terminate our program if the stream has 
        any errors; if there aren’t any errors, the program prints a message. We’ll add more functionality for the 
        success case in the next listing. The reason we might receive errors from the incoming method when a client 
        connects to the server is that we’re not actually iterating over connections. Instead, we’re iterating over 
        connection attempts. The connection might not be successful for a number of reasons, many of them operating 
        system specific. For example, many operating systems have a limit to the number of simultaneous open 
        connections they can support; new connection attempts beyond that number will produce an error until
        some of the open connections are closed.
        */
        let _stream = stream.unwrap();

        _pool.execute(|| 
            {
            handle_connection(_stream);
            });
        }
    println!("Connection established!");
    }

fn handle_connection(mut stream: TcpStream)
    {
    let mut buffer = [0; 1024];
    
    let get = b"GET /index HTTP/1.1\r\n";

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    if buffer.starts_with(get)
        {
        let contents = fs::read_to_string("public_html//index.html").unwrap();

        /* format!
        Next, we use format! to add the file’s contents as the body of the success response. 
        To ensure a valid HTTP response, we add the 
        Content-Length header which is set to the size of our response body, 
        in this case the size of hello.html.
        */
        let response = 
                format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
                );

        /*
        Because the write operation could fail, we use unwrap on any error result as before. 
        Again, in a real application you would add error handling here. 
        Finally, flush will wait and prevent the program from continuing until all the bytes 
        are written to the connection; 
        TcpStream contains an internal buffer to minimize calls to the underlying operating system.
         */
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        }
    else
        {
        // some other request.
        let status_line = "HTTP/1.1 404 NOT FOUND";
        stream.write(status_line.as_bytes()).unwrap();
        stream.flush().unwrap();
        }
    }