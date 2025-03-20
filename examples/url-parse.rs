use url::Url;

fn main() {
  let url_str = "http://bob:secret@sub.example.com:8080/somepath?foo=bar";
  match Url::parse(url_str) {
    Ok(url) => {
      println!("Scheme: {}", url.scheme());
      println!("Username: {}", url.username());
      println!("Password: {}", url.password().unwrap_or("None"));
      println!("Host: {}", url.host_str().unwrap_or("None"));
      println!("Port: {}", url.port().unwrap_or(0));
      println!("Path: {}", url.path());
      println!("Query: {}", url.query().unwrap_or("None"));
      println!("Fragment: {}", url.fragment().unwrap_or("None"));
    }
    Err(e) => {
      eprintln!("Failed to parse URL: {}", e);
    }
  }
}
