use anyhow::Result;
use kctf_pow::ChallengeParams;

/// Proof-of-Work struct that wraps kctf-pow functionality
pub struct Pow;

impl Pow {
    /// Generate a new challenge with the given difficulty
    pub fn generate(difficulty: u32) -> Result<String> {
        if difficulty == 0 {
            anyhow::bail!("Difficulty must be greater than 0");
        }

        let challenge_params = ChallengeParams::generate_challenge(difficulty);
        Ok(challenge_params.to_string())
    }

    /// Solve a challenge string
    pub fn solve(challenge: &str) -> Result<String> {
        let challenge_params = ChallengeParams::decode_challenge(challenge)
            .map_err(|e| anyhow::anyhow!("Failed to decode challenge: {}", e))?;
        
        let solution = challenge_params.solve();
        Ok(solution)
    }

    /// Verify a solution against a challenge
    pub fn verify(challenge: &str, solution: &str) -> Result<bool> {
        let challenge_params = ChallengeParams::decode_challenge(challenge)
            .map_err(|e| anyhow::anyhow!("Failed to decode challenge: {}", e))?;
        
        let is_valid = challenge_params.check(solution)
            .map_err(|e| anyhow::anyhow!("Failed to verify solution: {}", e))?;

        Ok(is_valid)
    }
}
