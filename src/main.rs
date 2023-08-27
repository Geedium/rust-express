use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use std::borrow::Cow;

use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn homepage(html: &mut String) {
    *html = html.replace("<!-- !INJECT! -->", "<script src='/js/client-bundle.js'></script>");
    *html = html.replace("<body id=\"root\"></body>", "<body id=\"root\"><p>Hello world!</p><button>Next Page</button></body>");
}

fn extract_class_name(jsx_code: &str) -> Option<String> {
    let re = Regex::new(r"export\s+default\s+function\s+(\w+)").unwrap();

    if let Some(caps) = re.captures(jsx_code) {
        if let Some(class_name) = caps.get(1) {
            return Some(class_name.as_str().to_string());
        }
    }

    None
}

fn extract_element_class_name(props: &str) -> Option<String> {
    let re = Regex::new(r#"\bclassName="([^"]*)""#).unwrap();

    if let Some(caps) = re.captures(props) {
        if let Some(class_name) = caps.get(1) {
            return Some(class_name.as_str().to_string());
        }
    }

    None
}

fn transform_jsx_to_js(jsx_code: &str) -> String {
    // Basic regex pattern for identifying JSX tags
    let re = Regex::new(r"<(\w+)\s*(.*?)>(.*?)</\w+>").unwrap();

    let mut mut_class_name = String::new();

    if let Some(class_name) = extract_class_name(jsx_code) {
        println!("Extracted class name: {}", class_name);
        mut_class_name = class_name;
    } else {
        println!("No export default class found.");
    }

    let mut js_code = String::new();
    // let cursor = 0;

    let mut tag = "";
    let mut props = "";
    let mut content = "";

    for cap in re.captures_iter(jsx_code) {
        // js_code.push_str(&jsx_code[cursor..cap.get(0).unwrap().start()]);

        tag = cap.get(1).unwrap().as_str();
        props = cap.get(2).unwrap().as_str();
        content = cap.get(3).unwrap().as_str();

        println!("Tags: {}", tag);
        println!("Props: {}", props);
        println!("Content: {}", content);
    }

    js_code.push_str(&format!(
        "class {} {{\n    _el = null;\n\n    constructor(props) {{\n        const node = document.createElement('{}');\n",
        // &jsx_code[open_tag_start + 1..open_tag_end], &jsx_code[open_tag_start + 1..open_tag_end]
        mut_class_name, tag
    ));

    // Extract className from props
    if let Some(class_name) = extract_element_class_name(props) {
        println!("Extracted style className: {}", class_name);
        js_code.push_str(&format!(
            "        node.className = \"{}\";\n    ",
            class_name
        ));
    } else {
        println!("No className found in props.");
    }

    if tag == "p" {
        js_code.push_str(&format!(
            "node.innerText = {}; \n", content.replace("{", "").replace("}", "")
        ));
    } else {
        js_code.push_str(&format!(
            "const text = document.createElement(\"p\");  \n  text.innerText = \"A Button\"; node.appendChild(text); \n"
        ));
    }

    if tag != "p" {
        js_code.push_str(&format!(
            "        node.addEventListener('click', props.onClick); \n  ",
        ));
    }

    js_code.push_str(&format!(
        "        this._el = node;\n    ",
    ));

    js_code.push_str(&format!(
        "}}\n\n    getNode() {{\n        return this._el;\n    }}\n}}"
    ));
    
    // while let Some(open_tag_start) = jsx_code[cursor..].find('<') {
        // let open_tag_start = cursor + open_tag_start;
        // let open_tag_end = open_tag_start + jsx_code[open_tag_start..].find('>').unwrap_or(0);

        // js_code.push_str(&jsx_code[cursor..open_tag_start]);

        // js_code.push_str(&format!(
        //     "class {} {{\n    _el = null;\n\n    constructor(props) {{\n        const node = document.createElement('{}');\n",
        //     &jsx_code[open_tag_start + 1..open_tag_end], &jsx_code[open_tag_start + 1..open_tag_end]
        // ));

        // js_code.push_str(&format!(
        //     "        this._el = node;\n    }}\n\n    getNode() {{\n        return this._el;\n    }}\n}}",
        // ));

        // if let Some(tag_name_end) = jsx_code[open_tag_start..open_tag_end].find(' ') {
        //     let tag_name = &jsx_code[open_tag_start + 1..open_tag_start + tag_name_end];
        //     let remaining = &jsx_code[open_tag_start + tag_name_end + 1..open_tag_end - 1];

        //     js_code.push_str(&format!(
        //         "class {} {{\n    _el = null;\n\n    constructor(props) {{\n        const node = document.createElement('{}');\n",
        //         tag_name, tag_name
        //     ));

        //     if !remaining.is_empty() {
        //         js_code.push_str(&format!("        node.className = '{}';\n", remaining));
        //     }

        //     js_code.push_str(&format!(
        //         "        this._el = node;\n    }}\n\n    getNode() {{\n        return this._el;\n    }}\n}}",
        //     ));

        // } else {
        //     js_code.push_str(&format!(
        //         "class {} {{\n    _el = null;\n\n    constructor(props) {{\n        const node = document.createElement('{}');\n",
        //         &jsx_code[open_tag_start + 1..open_tag_end], &jsx_code[open_tag_start + 1..open_tag_end]
        //     ));

        //     js_code.push_str(&format!(
        //         "        this._el = node;\n    }}\n\n    getNode() {{\n        return this._el;\n    }}\n}}",
        //     ));
        // }

        // cursor = open_tag_end + 1;
    // }

    // js_code.push_str(&jsx_code[cursor..]);
    return js_code
}

fn data(html: &mut String) -> Result<(), std::io::Error> {
    // Specify the path to the directory you want to read
    let dir_path = Path::new("./components");

   // Read the directory entries
   let entries = fs::read_dir(dir_path)?;

     // Loop through the directory entries
     for entry in entries {
        // Unwrap the entry or handle the error if it occurs
        let entry = entry?;

        // Get the path of the current entry
        let path = entry.path();

        // Check if the entry is a file or directory
        if entry.file_type()?.is_file() {
            println!("File: {:?}", path);

            // Get the filename without extension
            let file_stem = Path::new(&path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("unknown");

            // Define the desired extension
            let new_extension = "js";

            // Combine the filename and new extension
            let new_file_name = format!("js/{}.{}", file_stem, new_extension);

            println!("Original File Path: {}", path.display());
            println!("Transformed File Name: {}", new_file_name);
            
            let jsx_code = fs::read_to_string(path).expect("Failed to read file");
            let js_code = transform_jsx_to_js(&jsx_code);
            fs::write(new_file_name, js_code).expect("Failed to write file");
        } else if entry.file_type()?.is_dir() {
            println!("Directory: {:?}", path);
        } else {
            println!("Other: {:?}", path);
        }
    }

    *html = html.replace("<!-- !INJECT! -->", "<script src='/js/client-bundle.js'></script>");
    *html = html.replace("<body id=\"root\"></body>", "<body id=\"root\"><p>DATA!</p><button>Prev Page</button></body>");

    Ok(())
}

fn bundle(route: String, html: &mut String) {
    if route == "/" {
        homepage(html);
    }
    if route == "/data" {
        data(html);
    }
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET ";
    let post = b"POST ";
    let delete = b"DELETE ";

    let (method, url) = if buffer.starts_with(get) {
        ("GET", parse_url(&buffer, 4))
    } else if buffer.starts_with(post) {
        ("POST", parse_url(&buffer, 5))
    } else if buffer.starts_with(delete) {
        ("DELETE", parse_url(&buffer, 7))
    } else {
        ("", "")
    };
    
    let url = url.split_whitespace().next().unwrap_or("/");
    let url_ref = &url.to_string().split_off(1);
    println!("HTTP Method: {:?}, URL: {:?}", method, url);
    println!("URL REF: {:?}", url_ref);

    let js_directory = Path::new("js");

    if let Ok(entries) = fs::read_dir(js_directory) {
        let js_files: Vec<String> = entries
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".js") {
                            Some(file_name.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let mut response_tuples: Vec<(&str, Cow<str>, &str, Cow<str>, &str)> = Vec::new();
        response_tuples.push(("GET", std::borrow::Cow::Borrowed("/"), "HTTP/1.1 200 OK", std::borrow::Cow::Borrowed("index.html"), "text/html; charset=utf-8"));
        response_tuples.push(("GET", std::borrow::Cow::Borrowed("/data"), "HTTP/1.1 200 OK", std::borrow::Cow::Borrowed("data.html"), "text/html; charset=utf-8"));
        response_tuples.push(("GET", std::borrow::Cow::Borrowed("/styles.css"), "HTTP/1.1 200 OK", std::borrow::Cow::Borrowed("styles.css"), "text/css; charset=utf-8"));
        response_tuples.push(("GET", std::borrow::Cow::Borrowed("/mui.css"), "HTTP/1.1 200 OK", std::borrow::Cow::Borrowed("mui.css"), "text/css; charset=utf-8"));
    
        // Loop through js_files and do something for each file
        for js_file in &js_files {
            let s_format = format!("js/{}", js_file);
            let s_format_owned = s_format.clone();  // Create an owned String
            let s_slice: Cow<str> = s_format_owned.into();     // Take a slice of the owned String

            let s_format2 = format!("/js/{}", js_file);
            let s_format2_owned = s_format2.clone();
            let s_slice2: Cow<str> = s_format2_owned.into();     // Take a slice of the owned String

            println!("Processing JavaScript file: {}", s_slice);
            response_tuples.push(("GET", s_slice2.clone(), "HTTP/1.1 200 OK", s_slice.clone().into(), "application/javascript; charset=utf-8"));
        }

        let mut matched_response: Option<(&str, &str, &str)> = None;

        for (method_match, url_match, status_line, filename, content_type) in &response_tuples {
            if method == *method_match && url == *url_match {
                matched_response = Some((status_line, filename, content_type));
                break;
            }
        }

        if let Some((status_line, filename, content_type)) = matched_response {
            println!(
                "Status Line: {}\nFilename: {}\nContent Type: {}",
                status_line, filename, content_type
            );

            let mut contents = std::fs::read_to_string(filename).unwrap();
            // contents = contents.replace("{}", "{ \"success\": \"SSR\" }");
            // contents = contents.replace("{{ success }}", "SSR");
            
            bundle(url.to_string(), &mut contents);
                
            let mut response = format!(
                "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                content_type,
                contents
            );
    
            if content_type == "application/javascript; charset=utf-8" {
                response = format!(
                    "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nCache-Control: {}\r\n\r\n{}",
                    status_line,
                    contents.len(),
                    content_type,
                    // "max-age=2592000",
                    "no-cache",
                    contents
                );
            }
    
            if content_type == "text/css; charset=utf-8" {
                response = format!(
                    "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nCache-Control: {}\r\n\r\n{}",
                    status_line,
                    contents.len(),
                    content_type,
                    // "max-age=2592000",
                    "no-cache",
                    contents
                );
            }
            
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            println!(
                "Status Line: {}\nFilename: {}\nContent Type: {}",
                "HTTP/1.1 404 NOT FOUND",
                "404.html",
                "text/html; charset=utf-8"
            );

            let mut contents = std::fs::read_to_string("404.html").unwrap();

            let mut response = format!(
                "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                "HTTP/1.1 404 NOT FOUND",
                contents.len(),
                "text/html; charset=utf-8",
                contents
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn parse_url(buffer: &[u8], start_index: usize) -> &str {
    let end_index = buffer.iter().position(|&x| x == b'\r' || x == b'\n').unwrap_or_else(|| buffer.len());
    std::str::from_utf8(&buffer[start_index..end_index]).unwrap().trim()
}