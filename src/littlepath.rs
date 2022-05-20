use std::borrow::Borrow;
use std::fs;
use std::path::PathBuf;
use std::path::Component;
use std::str::FromStr;

struct Candidate {
    path: PathBuf,
    cost: usize,
}

impl Clone for Candidate {
    fn clone(&self) -> Self {
        Self {
          path: self.path.clone(),
          cost: self.cost.clone(),
        }
    }
}

pub fn resolve(query: PathBuf, relative_to: PathBuf) {
  let mut candidates: Vec<Candidate> = vec![
    Candidate {
      // Although this is the starting value, it might change based on the
      // configuration of the query component.
      path: relative_to,
      cost: 0,
    }
  ];

  for query_component in query.components() {
    let mut next_level_candidates: Vec<Candidate> = Vec::new();

    for ref candidate in candidates.to_owned() {
      // In case the query component is referring to the current directory, you
      // can safely ignore it, the candidates list can remain the same.
      if Component::CurDir == query_component {
        continue;
      }

      // In case the query component starts with a root, we need to reset the
      // candidate to to the root location.
      else if Component::RootDir == query_component {
        next_level_candidates.push(Candidate {
          path: PathBuf::from_str("/").unwrap(),
          cost: 0,
        });
      }

      // In case the query component is referring to parent directory (..), you just
      // need to append the parent to the current set of candidates.
      else if Component::ParentDir == query_component {
        let mut clone = candidate.clone();

        // TODO: This should not be necessary, to make it happen, the query
        // path should be normalized beforehand.
        clone.path.push("..");

        // Add updated candidate to the upcoming new candidates list.
        next_level_candidates.push(clone);
      }

      // In case the current query component is a valid entity (directory or file) name,
      // try to find the matching files.
      else if let Component::Normal(name) = query_component {
        let partial_name = name
            .to_owned()
            .into_string()
            .unwrap()
            .to_lowercase();

        let directory_listing = fs::read_dir(
          candidate.path.to_owned());

        if directory_listing.is_err() {
          continue;
        }

        let directory_walker = directory_listing
          .unwrap()
          .into_iter();

        for directory in directory_walker {
          let entity_name = directory
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();

          let matched = entity_name
            .to_lowercase()
            .find(&partial_name);

          if let Some(value) = matched {
            let mut clone = candidate.clone();

            clone.path.push(entity_name);
            clone.cost += value;

            next_level_candidates.push(clone);
          }
        }
      }
    }

    candidates = next_level_candidates;
  }

  candidates
    .sort_by(|a, b| a.cost.cmp(b.cost.borrow()));

  for item in candidates {
    match item.path.to_str() {
      Some(value) => println!("{}", value),
      None => (),
    }
  }
}
