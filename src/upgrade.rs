use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseState {
    Draft,
    Validating,
    Ready,
    Active,
    Failed(String),
    Retired,
}

#[derive(Debug, Clone)]
pub struct Release {
    pub id: String,
    pub version: String,
    pub state: ReleaseState,
}

impl Release {
    pub fn new(id: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            state: ReleaseState::Draft,
        }
    }
}

pub trait Validator: Send + Sync {
    fn validate(&self, release: &Release) -> Result<(), String>;
}

/// Always-passing validator (useful for tests / dev).
pub struct NoopValidator;
impl Validator for NoopValidator {
    fn validate(&self, _release: &Release) -> Result<(), String> {
        Ok(())
    }
}

pub struct UpgradeManager {
    current: Option<Release>,
    candidate: Option<Release>,
    validator: Box<dyn Validator>,
}

impl UpgradeManager {
    pub fn new(validator: Box<dyn Validator>) -> Self {
        Self {
            current: None,
            candidate: None,
            validator,
        }
    }

    pub fn current(&self) -> Option<&Release> {
        self.current.as_ref()
    }

    pub fn candidate(&self) -> Option<&Release> {
        self.candidate.as_ref()
    }

    /// Register v1 as the active release.
    pub fn activate_initial(&mut self, mut release: Release) {
        release.state = ReleaseState::Active;
        self.current = Some(release);
    }

    /// Submit a new feature/version as a candidate (does not replace current).
    pub fn submit_candidate(&mut self, mut release: Release) {
        release.state = ReleaseState::Validating;
        self.candidate = Some(release);
    }

    /// Run validation on the candidate. Promotes to Ready on success.
    pub fn validate_candidate(&mut self) -> Result<(), String> {
        let cand = self
            .candidate
            .as_mut()
            .ok_or_else(|| "no candidate".to_string())?;
        match self.validator.validate(cand) {
            Ok(()) => {
                cand.state = ReleaseState::Ready;
                Ok(())
            }
            Err(e) => {
                cand.state = ReleaseState::Failed(e.clone());
                Err(e)
            }
        }
    }

    /// Promote a validated candidate to active, retiring the previous release.
    /// Only succeeds if candidate is Ready.
    pub fn promote(&mut self) -> Result<(), String> {
        let cand = self
            .candidate
            .as_mut()
            .ok_or_else(|| "no candidate".to_string())?;
        if cand.state != ReleaseState::Ready {
            return Err(format!("candidate not ready: {:?}", cand.state));
        }
        cand.state = ReleaseState::Active;
        if let Some(prev) = self.current.as_mut() {
            prev.state = ReleaseState::Retired;
        }
        let promoted = self.candidate.take().unwrap();
        // retire previous
        if let Some(prev) = self.current.take() {
            let mut retired = prev;
            retired.state = ReleaseState::Retired;
            // keep retired in current? We replace current with promoted.
            let _ = retired;
        }
        self.current = Some(promoted);
        Ok(())
    }

    /// Discard the candidate without affecting the running release.
    pub fn discard_candidate(&mut self) {
        if let Some(c) = self.candidate.as_mut() {
            c.state = ReleaseState::Retired;
        }
        self.candidate = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staged_promotion() {
        let mut mgr = UpgradeManager::new(Box::new(NoopValidator));
        mgr.activate_initial(Release::new("r1", "1.0.0"));
        assert_eq!(mgr.current().unwrap().state, ReleaseState::Active);

        mgr.submit_candidate(Release::new("r2", "1.1.0"));
        assert_eq!(mgr.candidate().unwrap().state, ReleaseState::Validating);

        mgr.validate_candidate().unwrap();
        assert_eq!(mgr.candidate().unwrap().state, ReleaseState::Ready);

        mgr.promote().unwrap();
        assert_eq!(mgr.current().unwrap().version, "1.1.0");
        assert_eq!(mgr.current().unwrap().state, ReleaseState::Active);
        assert!(mgr.candidate().is_none());
    }

    #[test]
    fn failed_validation_blocks_promotion() {
        struct Bad;
        impl Validator for Bad {
            fn validate(&self, _r: &Release) -> Result<(), String> {
                Err("nope".into())
            }
        }
        let mut mgr = UpgradeManager::new(Box::new(Bad));
        mgr.activate_initial(Release::new("r1", "1.0.0"));
        mgr.submit_candidate(Release::new("r2", "1.1.0"));
        assert!(mgr.validate_candidate().is_err());
        assert!(mgr.promote().is_err());
        assert_eq!(mgr.current().unwrap().version, "1.0.0");
    }
}

impl fmt::Display for ReleaseState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseState::Draft => write!(f, "draft"),
            ReleaseState::Validating => write!(f, "validating"),
            ReleaseState::Ready => write!(f, "ready"),
            ReleaseState::Active => write!(f, "active"),
            ReleaseState::Failed(e) => write!(f, "failed: {e}"),
            ReleaseState::Retired => write!(f, "retired"),
        }
    }
}
