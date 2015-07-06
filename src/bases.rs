pub struct GitBase {
  pub repo: String,
  pub branch: Option<String>,
}

impl GitBase {
  pub fn git_url(&self) -> String {
    let mut url = self.repo.to_string();
    match self.branch {
      Some(ref b) => {
        url.push_str("#");
        url.push_str(&b);
      },
      None => ()
    };
    url
  }
}

pub fn get_base(base_url: &str) -> GitBase {
  let url_parts: Vec<&str> = base_url.splitn(2, "#").collect();
  GitBase {
    repo: url_parts.get(0).unwrap().to_string(),
    branch: url_parts.get(1).map(|s| { s.to_string() }),
  }
}
