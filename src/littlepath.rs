use std::fs;
use std::path::PathBuf;
use std::path::Component;

#[derive(Clone)]
struct Candidate {
    path: PathBuf,
    cost: usize,
}

impl Candidate {
    pub fn new() -> Candidate {
        return Candidate{
            path: PathBuf::new(),
            cost: 0,
        };
    }
}

pub fn resolve(query: PathBuf, relative_to: PathBuf) {
  let candidates: Vec<Candidate> = vec![
    Candidate{
      path: relative_to,
      cost: 0,
    }
  ];

  for query_component in query.components() {
    for candidate in candidates.to_owned() {
      if Component::CurDir == query_component {
        continue;
      }
      else if Component::ParentDir == query_component {
        let mut clone = candidate.clone();

        // TODO: This should not be necessary, to make it happen, the query
        // path should be normalized beforehand.
        clone.path.push("..");
      }
      else if let Component::Normal(name) = query_component {
        let partial_name = name
            .to_owned()
            .into_string()
            .unwrap();

        for file_result in fs::read_dir(candidate.path).unwrap().into_iter() {
          let file_name = file_result
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();

          let matched = file_name
            .to_lowercase()
            .find(&partial_name);

          if None == matched {
            println!("{:?} was not matched", file_name);
          }
          else if let Some(value) = matched {
            println!("{:?} matched with cost {:?}", file_name, value);
          }
        }
      }
    }
  }
}
